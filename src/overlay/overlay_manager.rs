use super::{
    address_peer_to_actor, database, debug, error,
    flood_gate::FloodGateActor,
    info,
    itertools::join,
    peer::{Peer, PeerActor, PeerInterface},
    riker::actors::*,
    xdr, AstroProtocol, CONFIG,
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
#[derive(Clone, Debug)]
pub struct OverlayManager {
    known_peer_adresses: HashSet<String>,
    authenticated_peers: HashSet<String>,
    pending_peers: HashSet<String>,
    failed_peers: HashSet<String>,
}

#[derive(Debug)]
pub enum OverlayError {
    AuthFail,
    ConnectionFail,
    InvalidPeerAddress,
}

impl OverlayManager {
    pub fn new() -> Self {
        OverlayManager {
            known_peer_adresses: HashSet::new(),
            authenticated_peers: HashSet::new(),
            pending_peers: HashSet::new(),
            failed_peers: HashSet::new(),
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
                let peer_format_name = format!("{}:{}", join(addr, "."), peer_addres.port);
                self.add_known_peer(peer_format_name);
            }
        }
    }

    /// Add new peer address to known_peer_adresses list
    fn add_known_peer(&mut self, peer_address: String) {
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

    /// Add single peer address to authenticated_peers list
    fn add_authenticated_peer(&mut self, peer_address: String) {
        self.authenticated_peers.insert(peer_address);
    }

    /// Remove single peer address from authenticated_peers list
    fn remove_authenticated_peer(&mut self, peer_address: &String) {
        self.authenticated_peers.remove(peer_address);
    }

    /// Add single peer address to failed_peer list
    fn add_failed_peer(&mut self, peer_address: String) {
        self.failed_peers.insert(peer_address);
    }

    /// Remove single peer address from failed_peer list
    fn remove_failed_peer(&mut self, peer_address: &String) {
        self.failed_peers.remove(peer_address);
    }

    fn is_new_peer(&self, peer_address: &String) -> bool {
        !self.is_peer_exist(peer_address)
    }

    fn is_peer_exist(&self, peer_address: &String) -> bool {
        self.known_peer_adresses.contains(peer_address)
            || self.authenticated_peers.contains(peer_address)
            || self.pending_peers.contains(peer_address)
            || self.failed_peers.contains(peer_address)
    }

    /// Move peer to authenticated_peers list
    fn move_peer_to_authenticated_list(&mut self, peers_addresses: String) {
        self.remove_known_peer(&peers_addresses);
        self.remove_pending_peer(&peers_addresses);
        self.add_authenticated_peer(peers_addresses);
    }

    fn move_peer_to_pending_list(&mut self, peers_address: String) {
        self.remove_known_peer(&peers_address);
        self.remove_authenticated_peer(&peers_address);
        self.remove_failed_peer(&peers_address);
        self.add_pending_peer(peers_address);
    }

    fn move_peer_to_failed_list(&mut self, peers_address: String) {
        self.remove_known_peer(&peers_address);
        self.remove_authenticated_peer(&peers_address);
        self.remove_pending_peer(&peers_address);
        self.add_failed_peer(peers_address);
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

    fn reached_limit_of_authenticated_peers(&self) -> bool {
        self.authenticated_peers.len() >= self.limit_authenticated_peers()
    }

    fn peers_to_authenticated_limit(&self) -> i32 {
        self.limit_authenticated_peers() as i32 - self.authenticated_peers.len() as i32
    }
}

// stub for current ledger value
fn unix_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

#[derive(Clone, Debug)]
pub(crate) struct OverlayManagerActor {
    state: OverlayManager,
}

impl OverlayManagerActor {
    pub fn new() -> BoxActor<AstroProtocol> {
        let actor = OverlayManagerActor {
            state: OverlayManager::new(),
        };

        Box::new(actor)
    }

    pub fn props() -> BoxActorProd<AstroProtocol> {
        Props::new(Box::new(OverlayManagerActor::new))
    }

    /// Run FloodGate
    pub fn run_flood_gate(&mut self, ctx: &Context<AstroProtocol>) {
        ctx.system
            .actor_of(FloodGateActor::props(), "flood_gate")
            .unwrap();
    }

    /// Run Listener actor for checking incoming connections
    pub fn run_listener_actor(&mut self, ctx: &Context<AstroProtocol>) {
        ctx.system
            .actor_of(OverlayListenerActor::props(), "overlay_connection_listener")
            .unwrap();
    }

    /// Run sheduler for checking minimal count of connections with peers
    pub fn run_periodic_checker(&mut self, ctx: &Context<AstroProtocol>) {
        let delay = Duration::from_millis(1000);
        ctx.schedule_once(
            delay,
            ctx.myself(),
            None,
            AstroProtocol::CheckOverlayMinConnectionsCmd,
        );
    }

    /// Check minimal connections
    pub fn check_min_connections(&mut self, ctx: &Context<AstroProtocol>) {
        if self.state.reached_limit_of_authenticated_peers() {
            return;
        }

        let limit = self.state.peers_to_authenticated_limit() as usize;
        let mut taked_peers: Vec<_> = self
            .state
            .known_peer_adresses
            .iter()
            .cloned()
            .take(limit)
            .collect();

        for peer in taked_peers {
            self.state.move_peer_to_pending_list(peer.to_owned());
            self.handle_new_initiated_peer(ctx, peer);
        }

        self.run_periodic_checker(ctx);
    }

    pub fn handle_new_incoming_peer(&mut self, ctx: &Context<AstroProtocol>, mut peer: Peer) {
        if !self.state.reached_limit_of_authenticated_peers() {
            let name = format!("peer-{}", address_peer_to_actor(peer.peer_addr()));
            ctx.system
                .actor_of(PeerActor::incoming_peer_props(peer), &name);
        }
    }

    pub fn handle_new_initiated_peer(&mut self, ctx: &Context<AstroProtocol>, address: String) {
        let name = format!("peer-{}", address_peer_to_actor(address.clone()));
        ctx.system
            .actor_of(PeerActor::initiated_peer_props(address), &name);
    }

    pub fn handle_incoming_message(
        &mut self,
        ctx: &Context<AstroProtocol>,
        address: String,
        message: xdr::StellarMessage,
    ) {
        match message {
            xdr::StellarMessage::Peers(ref set_of_peers) => {
                self.state.add_known_peers(set_of_peers);
                info!(
                    "[OverlayManager][add_known_peers] {:?}",
                    self.state.known_peer_adresses
                );
            }
            xdr::StellarMessage::Transaction(_) | xdr::StellarMessage::Envelope(_) => {
                let flood_gate = ctx.select("/user/flood_gate").unwrap();
                flood_gate.tell(
                    AstroProtocol::AddRecordFloodGateCmd(message.to_owned(), address, unix_time()),
                    None,
                );
                flood_gate.tell(
                    AstroProtocol::BroadcastFloodGateCmd(
                        message.to_owned(),
                        false,
                        self.state.authenticated_peers.clone(),
                    ),
                    None,
                );
                flood_gate.tell(AstroProtocol::ClearFloodGateCmd(unix_time()), None);
            }
            _ => (),
        }
    }
}

impl Actor for OverlayManagerActor {
    type Msg = AstroProtocol;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        self.state = OverlayManager::new();
        self.state.populate_known_peers_from_db();
    }

    fn receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        sender: Option<ActorRef<Self::Msg>>,
    ) {
        debug!("OVERLAY RECEIVE: {:?}", msg);
        match msg {
            AstroProtocol::CheckOverlayMinConnectionsCmd => self.check_min_connections(ctx),
            AstroProtocol::HandleOverlayIncomingPeerCmd(peer) => {
                self.handle_new_incoming_peer(ctx, peer)
            }
            AstroProtocol::ReceivedPeerMessageCmd(address, message) => {
                self.handle_incoming_message(ctx, address, message)
            }
            AstroProtocol::AuthPeerOkCmd(address) => {
                self.state.move_peer_to_authenticated_list(address)
            }
            AstroProtocol::FailedPeerCmd(address) => {
                self.state.move_peer_to_failed_list(address);
                ctx.system.stop(&sender.unwrap());
            }
            _ => unreachable!(),
        }
    }

    fn post_start(&mut self, ctx: &Context<Self::Msg>) {
        self.run_listener_actor(ctx);
        self.run_periodic_checker(ctx);
        self.run_flood_gate(ctx);
    }
}

pub(crate) struct OverlayListenerActor;

impl OverlayListenerActor {
    pub fn new() -> BoxActor<AstroProtocol> {
        Box::new(OverlayListenerActor)
    }

    fn props() -> BoxActorProd<AstroProtocol> {
        Props::new(Box::new(OverlayListenerActor::new))
    }
}

impl Actor for OverlayListenerActor {
    type Msg = AstroProtocol;

    fn receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        _sender: Option<ActorRef<Self::Msg>>,
    ) {
        unreachable!();
    }

    fn post_start(&mut self, ctx: &Context<Self::Msg>) {
        let listener = TcpListener::bind(CONFIG.local_node().address()).expect(
            "[Overlay][Listener] Unable to listen local address to handle incoming connections",
        );

        info!(
            "[Overlay][Listener] start to listen incoming connections on {:?}",
            CONFIG.local_node().address()
        );

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let peer = Peer::new(stream, CONFIG.local_node().address());
                    ctx.myself()
                        .parent()
                        .tell(AstroProtocol::HandleOverlayIncomingPeerCmd(peer), None)
                }
                Err(e) => {
                    info!("[Overlay][Listener] CONNECTION FAILED, cause: {:?}", e);
                }
            }
        }
        unreachable!();
    }
}
