use crate::overlay::Peer;
use crate::xdr;
use riker::actors::*;

/**
 * AstroProtocol contains all kind of messages we use for
 * communication between actors
*/

#[derive(Clone, Debug)]
pub enum AstroProtocol {
    /// New Peer incoming from Listener
    HandleOverlayIncomingPeerCmd(Peer),
    /// Check connections limit
    CheckOverlayMinConnectionsCmd,
    /// Awaiting incoming messag from remote peer
    ServePeerCmd,
    /// PeerActor must send message
    SendPeerMessageCmd(xdr::StellarMessage),
    /// Received message from PeerActor
    ReceivedPeerMessageCmd(String, xdr::StellarMessage),
    /// Authentication process between PeerActor and remote peer succeed
    AuthPeerOkCmd(String),
    /// PeerActor interaction failed
    FailedPeerCmd(String),
}

impl Into<ActorMsg<AstroProtocol>> for AstroProtocol {
    fn into(self) -> ActorMsg<AstroProtocol> {
        ActorMsg::User(self)
    }
}
