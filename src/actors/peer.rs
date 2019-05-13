use super::{debug, riker::actors::*, AstroProtocol, Peer, PeerInterface};
use std::time::Duration;

#[derive(Debug)]
pub struct PeerActor {
    address: Option<String>,
    peer: Option<Peer>,
}

impl PeerActor {
    pub fn new((address, peer): (Option<String>, Option<Peer>)) -> BoxActor<AstroProtocol> {
        let actor = PeerActor { address, peer };

        Box::new(actor)
    }

    pub fn initiated_peer_props(address: String) -> BoxActorProd<AstroProtocol> {
        Props::new_args(Box::new(PeerActor::new), (Some(address), None))
    }

    pub fn incoming_peer_props(peer: Peer) -> BoxActorProd<AstroProtocol> {
        Props::new_args(Box::new(PeerActor::new), (None, Some(peer)))
    }

    pub fn overlay_manager_ref(
        &self,
        ctx: &Context<AstroProtocol>,
    ) -> ActorSelection<AstroProtocol> {
        ctx.select("/user/overlay_manager").unwrap()
    }

    pub fn repeateable_read(&self, ctx: &Context<AstroProtocol>) {
        let delay = Duration::from_millis(200);
        ctx.schedule_once(delay, ctx.myself(), None, AstroProtocol::ServePeerCmd);
    }
}

impl Actor for PeerActor {
    type Msg = AstroProtocol;

    fn receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        _sender: Option<ActorRef<Self::Msg>>,
    ) {
        match msg {
            AstroProtocol::ServePeerCmd => {
                match self.peer.as_mut().unwrap().receive_message() {
                    Ok(msg) => {
                        self.overlay_manager_ref(ctx).tell(
                            AstroProtocol::ReceivedPeerMessageCmd(
                                self.address.as_ref().unwrap().to_owned(),
                                msg.into(),
                            ),
                            Some(ctx.myself()),
                        );
                        self.repeateable_read(ctx);
                    }
                    Err(e) => {
                        debug!("Cant read XDR message cause: {}", e);
                        self.overlay_manager_ref(ctx).tell(
                            AstroProtocol::FailedPeerCmd(self.address.as_ref().unwrap().to_owned()),
                            Some(ctx.myself()),
                        );
                    }
                };
            }
            AstroProtocol::SendPeerMessageCmd(message) => {
                self.peer.as_mut().unwrap().send_message(message);
            }
            _ => unreachable!(),
        }
    }

    fn post_start(&mut self, ctx: &Context<Self::Msg>) {
        if self.address.is_some() && self.peer.is_none() {
            let result = Peer::connect_to(self.address.as_ref().unwrap().to_owned());
            if result.is_ok() {
                let mut peer = result.unwrap();
                peer.start_authentication(true);
                if peer.is_authenticated() {
                    self.peer = Some(peer);
                }
            }
        } else if self.address.is_none() && self.peer.is_some() {
            self.peer.as_mut().unwrap().start_authentication(false);
        } else {
            unreachable!()
        }

        if let Some(ref peer) = self.peer {
            if peer.is_authenticated() {
                self.overlay_manager_ref(ctx).tell(
                    AstroProtocol::AuthPeerOkCmd(self.address.as_ref().unwrap().to_owned()),
                    Some(ctx.myself()),
                );
                self.repeateable_read(ctx);
                return;
            }
        }
        self.overlay_manager_ref(ctx).tell(
            AstroProtocol::FailedPeerCmd(self.address.as_ref().unwrap().to_owned()),
            Some(ctx.myself()),
        );
    }
}
