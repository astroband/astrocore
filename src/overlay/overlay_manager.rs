use crate::network;
use crate::overlay::peer::{Peer, PeerInterface};
use crate::overlay::flood_gate::{FloodGate};
use crate::scp::local_node::LocalNode;
use crate::xdr;
use itertools::join;
use log::{error, info};
use std::net::TcpStream;
use std::time::Duration;
use std::collections::{HashSet, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

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
    local_node: LocalNode,
}

const SECRET_TEST_SEED: &str = "SATKBWSRLPHGM2FKMLZ4QNH64XYKP7J2O6U5QNFBJHYEXDSHN75R5MVE";

#[derive(Debug)]
pub enum OverlayError {
    AuthFail,
    ConnectionFail,
    InvalidPeerAddress,
}

impl OverlayManager {
    pub fn new() -> Self {
        let seed = String::from(SECRET_TEST_SEED);
        let node = LocalNode::new(seed, &network::Network::test_network().network_id());

        OverlayManager {
            known_peer_adresses: HashSet::new(),
            authenticated_peers: HashMap::new(),
            pending_peers: HashSet::new(),
            local_node: node,
        }
    }

    /// start main overlay manager process
    pub fn start(&mut self) {
        let mut peer = self
            .connect_to(get_initial_peer_address())
            .expect("Initial peer connection failed.");

        loop {
            let message_content = peer.receive_message();
            match message_content {
                Ok(msg) => {
                    match msg {
                        xdr::AuthenticatedMessage::V0 {
                            0:
                                xdr::AuthenticatedMessageV0 {
                                    message: xdr::StellarMessage::Peers(ref peers_vec),
                                    ..
                                },
                        } => {
                            self.add_peers(peers_vec);
                            self.auth_all_known_peers();
                            break
                        }
                        _ => info!("\n{:?}", msg),
                    };
                }
                Err(e) => error!("Cant read XDR message cause: {}", e),
            };
        }
        self.add_peer_to_authenticated_list(get_initial_peer_address(), peer);

        let mut flood_gate = FloodGate::new();

        loop {
            let unix_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32;

            let mut err_flag: (bool, String) = (false, String::from(""));
            info!("Auth peers: {:?}", self.authenticated_peers.keys());
            {
                for (peer_address, peer) in &mut self.authenticated_peers {
                    let message_content = peer.receive_message();
                    match message_content {
                        Ok(xdr::AuthenticatedMessage::V0 {
                            0:
                                xdr::AuthenticatedMessageV0 {
                                    message: ref msg,
                                    ..
                                },
                        }) => {
                            info!("Received message from: {}", peer_address);
                            flood_gate.add_record(msg, peer_address.clone(), unix_time);
                            // flood_gate.broadcast(msg.clone(), false, self.authenticated_peers_values()[..];
                        },
                        Err(e) => {
                            error!("Cant read XDR message cause: {}", e);
                            err_flag = (true, peer_address.clone());
                        },
                    };
                }
            }
            if err_flag.0 {
                self.remove_peer_from_authenticated_list(&err_flag.1);
                // self.restore_number_of_peers()
            }
        }
    }

    /// Accept peer_address in parseable format and trying to start_authenticate new connection
    fn connect_to(&self, peer_address: String) -> Result<Peer, OverlayError> {
        let address = match peer_address.parse() {
            Ok(addr) => addr,
            Err(_) => return Err(OverlayError::InvalidPeerAddress),
        };

        match TcpStream::connect_timeout(&address, Duration::new(2, 0)) {
            Ok(stream) => {
                info!("Successfully connected to peer {}", address);
                let cloned_stream = stream.try_clone().expect("clone failed...");

                let mut peer = Peer::new(self.local_node.clone(), cloned_stream, peer_address);
                peer.start_authentication();

                if peer.is_authenticated() {
                    return Ok(peer);
                } else {
                    return Err(OverlayError::AuthFail);
                }
            }
            Err(e) => {
                error!("Failed to connect: {}, cause {}", address, e);
                return Err(OverlayError::ConnectionFail);
            }
        }
    }

    /// Accept llist of xdr::PeerAddress and move each of them in known_peer_adresses
    /// in parseable format
    fn add_peers(&mut self, peers_addresses: &Vec<xdr::PeerAddress>) {
        for peer_addres in peers_addresses {
            if let xdr::PeerAddress{ip: xdr::PeerAddressIp::Ipv4(ref addr), ..} = peer_addres {
                self.known_peer_adresses.insert(format!("{}:{}", join(addr, "."), peer_addres.port));
            }
        }
    }

    /// Iterate through known_peer_adresses and trying to a authenticate each.
    /// If authentication procees is Ok(), so move new peer in list of authenticated_peers
    fn auth_all_known_peers(&mut self) {
        let peers_address = self.known_peer_adresses.clone();
        for peer_address in peers_address {
            if self.authenticated_peers.len() > self.limit_authenticated_peers() {
                break
            };

            if self.authenticated_peers.contains_key(&peer_address) {
                continue
            };

            let peer_result = self.connect_to(peer_address.clone());
            if let Ok(peer) = peer_result {
                self.add_peer_to_authenticated_list(peer_address.clone(), peer);    
            };
        }
    }

    /// Add new peer to authenticated_peers list
    fn add_peer_to_authenticated_list(&mut self, peers_addresses: String, peer: Peer) {
        self.authenticated_peers.insert(peers_addresses, peer);
    }

    fn remove_peer_from_authenticated_list(&mut self, peers_addresses: &String) {
        self.authenticated_peers.remove(peers_addresses);
    }

    /// Limit number of connections we can have between peers
    fn limit_authenticated_peers(&self) -> usize {
        // TODO: Get this value from config
        5
    }

    // fn authenticated_peers_values(&self) -> Vec<Peer> {
    //     let mut values: Vec<Peer> = vec![];
    //     for val in self.authenticated_peers.values() {
    //         values.push(*val.clone())
    //     };
    //     values
    // }
}

/// TODO: temp test address
fn get_initial_peer_address() -> String {
    String::from("54.160.175.7:11625")
}
