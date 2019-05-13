use super::{database, itertools::join, xdr, CONFIG};
use std::collections::HashSet;
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
    pub(crate) fn new() -> Self {
        OverlayManager {
            known_peer_adresses: HashSet::new(),
            authenticated_peers: HashSet::new(),
            pending_peers: HashSet::new(),
            failed_peers: HashSet::new(),
        }
    }

    /// Accept llist of xdr::PeerAddress and move each of them in known_peer_adresses
    /// in parseable format
    pub(crate) fn add_known_peers(&mut self, peers_addresses: &[xdr::PeerAddress]) {
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
    pub(crate) fn add_known_peer(&mut self, peer_address: String) {
        if self.is_new_peer(&peer_address) {
            self.known_peer_adresses.insert(peer_address);
        }
    }

    /// Remove single peer address from known_peer_adresses list
    pub(crate) fn remove_known_peer(&mut self, peer_address: &str) {
        self.known_peer_adresses.remove(peer_address);
    }

    /// Add single peer address to pending_peers list
    pub(crate) fn add_pending_peer(&mut self, peer_address: String) {
        self.pending_peers.insert(peer_address);
    }

    /// Remove single peer address from pending_peers list
    pub(crate) fn remove_pending_peer(&mut self, peer_address: &str) {
        self.pending_peers.remove(peer_address);
    }

    /// Add single peer address to authenticated_peers list
    pub(crate) fn add_authenticated_peer(&mut self, peer_address: String) {
        self.authenticated_peers.insert(peer_address);
    }

    /// Remove single peer address from authenticated_peers list
    pub(crate) fn remove_authenticated_peer(&mut self, peer_address: &str) {
        self.authenticated_peers.remove(peer_address);
    }

    /// Add single peer address to failed_peer list
    pub(crate) fn add_failed_peer(&mut self, peer_address: String) {
        self.failed_peers.insert(peer_address);
    }

    /// Remove single peer address from failed_peer list
    pub(crate) fn remove_failed_peer(&mut self, peer_address: &str) {
        self.failed_peers.remove(peer_address);
    }

    pub(crate) fn is_new_peer(&self, peer_address: &str) -> bool {
        !self.is_peer_exist(peer_address)
    }

    pub(crate) fn is_peer_exist(&self, peer_address: &str) -> bool {
        self.known_peer_adresses.contains(peer_address)
            || self.authenticated_peers.contains(peer_address)
            || self.pending_peers.contains(peer_address)
            || self.failed_peers.contains(peer_address)
    }

    /// Move peer to authenticated_peers list
    pub(crate) fn move_peer_to_authenticated_list(&mut self, peers_addresses: String) {
        self.remove_known_peer(&peers_addresses);
        self.remove_pending_peer(&peers_addresses);
        self.add_authenticated_peer(peers_addresses);
    }

    pub(crate) fn move_peer_to_pending_list(&mut self, peers_address: String) {
        self.remove_known_peer(&peers_address);
        self.remove_authenticated_peer(&peers_address);
        self.remove_failed_peer(&peers_address);
        self.add_pending_peer(peers_address);
    }

    pub(crate) fn move_peer_to_failed_list(&mut self, peers_address: String) {
        self.remove_known_peer(&peers_address);
        self.remove_authenticated_peer(&peers_address);
        self.remove_pending_peer(&peers_address);
        self.add_failed_peer(peers_address);
    }

    /// Limit number of connections we can have between peers
    pub(crate) fn limit_authenticated_peers(&self) -> usize {
        *CONFIG.peers_limit() as usize
    }

    pub(crate) fn populate_known_peers_from_db(&mut self) {
        let peers = database::Peer::all().expect("[Overlay Manager] Can`t recreate initial peer adresses. Check your database connection");
        for peer in peers {
            self.add_known_peer(peer.address().to_owned());
        }
    }

    pub(crate) fn reached_limit_of_authenticated_peers(&self) -> bool {
        self.authenticated_peers.len() >= self.limit_authenticated_peers()
    }

    pub(crate) fn peers_to_authenticated_limit(&self) -> i32 {
        self.limit_authenticated_peers() as i32 - self.authenticated_peers.len() as i32
    }

    pub(crate) fn known_peer_adresses(&self) -> &HashSet<String> {
        &self.known_peer_adresses
    }

    pub(crate) fn authenticated_peers(&self) -> &HashSet<String> {
        &self.authenticated_peers
    }

    pub(crate) fn pending_peers(&self) -> &HashSet<String> {
        &self.pending_peers
    }

    pub(crate) fn failed_peers(&self) -> &HashSet<String> {
        &self.failed_peers
    }
}

// stub for current ledger value
fn unix_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
