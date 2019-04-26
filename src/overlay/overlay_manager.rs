use super::{
    crossbeam_channel::{unbounded, Receiver, Sender},
    database, error,
    flood_gate::FloodGate,
    info,
    itertools::join,
    peer::{Peer, PeerInterface},
    xdr, CONFIG,
};
use std::collections::{HashMap, HashSet};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
/**
 * OverlayManager maintains a virtual broadcast network, consisting of a set of
 * remote TCP peers (TCPPeer), a mechanism for flooding messages to all peers
 * (FloodGate), and a mechanism for sending and receiving anycast request/reply
 * pairs (ItemFetcher).
 *
 * Overlay network messages are defined as the XDR structure type
 * `StellarMessage`, in the file src/xdr/Stellar-overlay.x
 *
 * They are minimally framed using the Record Marking (RM) standard of RFC5531
 * (https://tools.ietf.org/html/rfc5531#page-16) and the RM-framed messages are
 * transmitted over TCP/IP sockets, between peers.
 *
 * The `StellarMessage` union contains 3 logically distinct kinds of message:
 *
 *  - Messages directed to or from a specific peer, with or without a response:
 *    HELLO, GET_PEERS, PEERS, DONT_HAVE, ERROR_MSG
 *
 *  - One-way broadcast messages informing other peers of an event:
 *    TRANSACTION and SCP_MESSAGE
 *
 *  - Two-way anycast messages requesting a value (by hash) or providing it:
 *    GET_TX_SET, TX_SET, GET_SCP_QUORUMSET, SCP_QUORUMSET, GET_SCP_STATE
 *
 * Anycasts are initiated and serviced two instances of ItemFetcher
 * (mTxSetFetcher and mQuorumSetFetcher). Anycast messages are sent to
 * directly-connected peers, in sequence until satisfied. They are not
 * flooded between peers.
 *
 * Broadcasts are initiated by the Herder and sent to both the Herder _and_ the
 * local FloodGate, for propagation to other peers.
 *
 * The OverlayManager tracks its known peers in the Database and shares peer
 * records with other peers when asked.
 */
pub struct OverlayManager {
    known_peer_adresses: HashSet<String>,
    authenticated_peers: HashMap<String, Peer>,
    pending_peers: HashSet<String>,
}

#[derive(Debug)]
pub enum OverlayError {
    AuthFail,
    ConnectionFail,
    InvalidPeerAddress,
}

pub enum OverlayMessages {
    NewConnection(String),
    NewPeer(String),
    PeerFailure(String),
    NewStellarMessage(String, xdr::StellarMessage),
}

impl OverlayManager {
    pub fn new() -> Self {
        //  RWlock
        OverlayManager {
            known_peer_adresses: HashSet::new(),
            authenticated_peers: HashMap::new(),
            pending_peers: HashSet::new(),
        }
    }

    /// start main overlay manager process
    pub fn start(&mut self) {
        let (overlay_sender_ch, overlay_receive_ch) =
            crossbeam_channel::unbounded::<OverlayMessages>();

        thread::spawn(move || handle_clients(overlay_sender_ch.clone()));

        self.populate_known_peers_from_db();

        let mut flood_gate = FloodGate::new();
        let mut received_messages: HashMap<String, xdr::StellarMessage> = HashMap::new();
        let mut failed_peers: HashSet<String> = HashSet::new();

        loop {
            self.auth_all_known_peers();
            info!("Auth peers: {:?}", self.authenticated_peers.keys());

            for (peer_address, peer) in &mut self.authenticated_peers {
                match peer.receive_message() {
                    Ok(msg) => {
                        let message: xdr::StellarMessage = msg.into();
                        info!("Received message from: {}", peer_address);
                        received_messages.insert(peer_address.clone(), message.clone());
                    }
                    Err(e) => {
                        error!("Cant read XDR message cause: {}", e);
                        failed_peers.insert(peer_address.clone());
                    }
                };
            }

            let current_ledger = unix_time();
            for (addr, msg) in received_messages.drain().take(1) {
                match msg {
                    xdr::StellarMessage::Peers(ref message) => self.add_known_peers(message),
                    xdr::StellarMessage::Transaction(_) | xdr::StellarMessage::Envelope(_) => {
                        flood_gate.add_record(&msg, addr.clone(), current_ledger);
                        flood_gate.broadcast(msg.clone(), false, &mut self.authenticated_peers);
                    }
                    _ => (),
                }
            }

            flood_gate.clear_below(current_ledger);

            for addr in failed_peers.drain().take(1) {
                self.remove_peer_from_authenticated_list(&addr);
            }
        }
    }

    /// Accept peer_address in parseable format and trying to start_authenticate new connection
    fn connect_to(&self, peer_address: &String) -> Result<Ok, OverlayError> {
        match TcpStream::connect_timeout(peer_address.parse(), Duration::new(2, 0)) {
            Ok(stream) => {
                launch_new_peer(stream, overlay_ch.clone(), true);
                // info!("Successfully connected to peer {}", address);
                // let cloned_stream = stream.try_clone().expect("clone failed...");

                // let mut peer = Peer::new(cloned_stream, peer_address);
                // peer.start_authentication(true);

                // if peer.is_authenticated() {
                //     return Ok(peer);
                // } else {
                //     return Err(OverlayError::AuthFail);
                // }
            }
            Err(e) => {
                info!("Failed to connect: {}, cause {}", peer_address, e);
                return Err(OverlayError::ConnectionFail);
            }
        }
    }

    /// Accept llist of xdr::PeerAddress and move each of them in known_peer_adresses
    /// in parseable format
    fn add_known_peers(&mut self, peers_addresses: &Vec<xdr::PeerAddress>) {
        for peer_addres in peers_addresses {
            if let xdr::PeerAddress {
                ip: xdr::PeerAddressIp::Ipv4(ref addr),
                ..
            } = peer_addres
            {
                self.add_known_peer(format!("{}:{}", join(addr, "."), peer_addres.port));
            }
        }
    }

    /// Add single peer address to known_peer_adresses list
    fn add_known_peer(&mut self, peer_address: String) {
        if let Err(_) = peer_address.parse() {
            return;
        }

        if self.is_new_peer(&peer_address) {
            self.known_peer_adresses.insert(peer_address);
        }
    }

    /// Remove single peer address from known_peer_adresses list
    fn remove_known_peer(&mut self, peer_address: &String) {
        self.known_peer_adresses.remove(peer_address);
    }

    /// Add single peer address to pending_peers list
    fn add_pending_peer(&mut self, peer_address: String) {
        self.pending_peers.insert(peer_address);
    }

    /// Remove single peer address from pending_peers list
    fn remove_pending_peer(&mut self, peer_address: &String) {
        self.pending_peers.remove(peer_address);
    }

    fn is_new_peer(&self, peer_address: &String) -> bool {
        !self.is_peer_exist(peer_address)
    }

    fn is_peer_exist(&self, peer_address: &String) -> bool {
        self.known_peer_adresses.contains(peer_address)
            || self.authenticated_peers.contains_key(peer_address)
            || self.pending_peers.contains(peer_address)
    }

    /// Iterate through known_peer_adresses and trying to a authenticate each.
    /// If authentication procees is Ok(), so move new peer in list of authenticated_peers
    fn auth_all_known_peers(&mut self) {
        if self.reached_limit_of_authenticated_peers() {
            return;
        };

        let peers_address = self.known_peer_adresses.clone();
        for peer_address in peers_address {
            if self.authenticated_peers.contains_key(&peer_address) {
                continue;
            };

            let peer_result = self.connect_to(&peer_address);
            // if let Ok(peer) = peer_result {
            //     self.move_peer_to_authenticated_list(peer_address.to_owned(), peer);
            // } else {
            //     self.move_peer_to_pending_list(peer_address.to_owned());
            // };

            // if self.reached_limit_of_authenticated_peers() {
            //     break;
            // };
        }
    }

    /// Move peer to authenticated_peers list
    fn move_peer_to_authenticated_list(&mut self, peers_addresses: String, peer: Peer) {
        self.remove_known_peer(&peers_addresses);
        self.authenticated_peers.insert(peers_addresses, peer);
    }

    /// Remove single peer address from authenticated_peers list
    fn remove_peer_from_authenticated_list(&mut self, peers_addresses: &String) {
        self.authenticated_peers.remove(peers_addresses);
    }

    /// Limit number of connections we can have between peers
    fn limit_authenticated_peers(&self) -> usize {
        *CONFIG.peers_limit() as usize
    }

    fn populate_known_peers_from_db(&mut self) {
        let peers = database::Peer::all().expect("[Overlay Manager] Can`t recreate initial peer adresses. Check your database connection");
        for peer in peers {
            self.add_known_peer(peer.address().to_owned());
        }
    }

    fn move_peer_to_pending_list(&mut self, peers_address: String) {
        self.remove_known_peer(&peers_address);
    }

    fn reached_limit_of_authenticated_peers(&self) -> bool {
        self.authenticated_peers.len() >= self.limit_authenticated_peers()
    }
}

// stub for current ledger value
fn unix_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

fn handle_clients(overlay_ch: crossbeam_channel::Sender<OverlayMessages>) {
    let listener = TcpListener::bind(CONFIG.local_node().address())
        .expect("Unable to send on OverlayManager channel");;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => launch_new_peer(stream, overlay_ch.clone(), false),
            Err(e) => {
                error!("CONNECTION FAILED, cause: {:?}", e);
            }
        }
    }
    unreachable!();
}

fn launch_new_peer(stream: TcpStream, overlay_ch: crossbeam_channel::Sender<OverlayMessages>, we_called_remote: bool) {
    thread::spawn(move || {
        let mut peer = Peer::new(stream, CONFIG.local_node().address());
        overlay_ch
            .send(OverlayMessages::NewConnection(peer.peer_addr()))
            .expect("Unable to send on OverlayManager channel");;

        peer.start_authentication(we_called_remote);
        if peer.is_authenticated() {
            overlay_ch
                .send(OverlayMessages::NewPeer(peer.peer_addr()))
                .expect("Unable to send on OverlayManager channel");
            peer.start_serve(overlay_ch);
        } else {
            overlay_ch
                .send(OverlayMessages::PeerFailure(peer.peer_addr()))
                .expect("Unable to send on OverlayManager channel");;
        }
    });
}
