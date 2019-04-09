use crate::network;
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
use crate::overlay::peer::{Peer, PeerInterface};
use crate::scp::local_node::LocalNode;
use log::{debug, error, info};
use std::net::TcpStream;
use std::time::Duration;

const SECRET_TEST_SEED: &str = "SATKBWSRLPHGM2FKMLZ4QNH64XYKP7J2O6U5QNFBJHYEXDSHN75R5MVE";

#[derive(Debug)]
pub enum OverlayError {
    AuthFail,
    ConnectionFail,
    InvalidPeerAddress,
}

pub struct OverlayManager {
    known_peer_adresses: Vec<String>,
    authenticated_peers: Vec<Peer>,
    pending_peers: Vec<Peer>,
    local_node: LocalNode,
}

impl OverlayManager {
    pub fn new() -> Self {
        let seed = String::from(SECRET_TEST_SEED);
        let node = LocalNode::new(seed, &network::Network::test_network().network_id());

        OverlayManager {
            known_peer_adresses: vec![],
            authenticated_peers: vec![],
            pending_peers: vec![],
            local_node: node,
        }
    }

    pub fn start(&self) {
        let mut peer = self
            .connect_to(get_initial_peer_address())
            .expect("Initial peer connection failed.");

        loop {
            let message_content = peer.receive_message();
            match message_content {
                Ok(msg) => info!("\n{:?}", msg),
                Err(e) => error!("Cant read XDR message cause: {}", e),
            };
        }
    }

    pub fn connect_to(&self, peer_address: String) -> Result<Peer, OverlayError> {
        let address = match peer_address.parse() {
            Ok(addr) => addr,
            Err(_) => return Err(OverlayError::InvalidPeerAddress),
        };

        match TcpStream::connect_timeout(&address, Duration::new(5, 0)) {
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
                error!("Failed to connect: {}", address);
                return Err(OverlayError::ConnectionFail);
            }
        }
    }
}

/// TODO: temp test address
fn get_initial_peer_address() -> String {
    String::from("54.160.175.7:11625")
}
