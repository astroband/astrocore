use super::{info,
    overlay_manager_ref,
    riker::actors::*,
    AstroProtocol,
    Peer,
    CONFIG,
};
use std::net::TcpListener;

pub(crate) struct OverlayListenerActor;

impl OverlayListenerActor {
    pub fn new() -> BoxActor<AstroProtocol> {
        Box::new(OverlayListenerActor)
    }

    pub fn props() -> BoxActorProd<AstroProtocol> {
        Props::new(Box::new(OverlayListenerActor::new))
    }
}

impl Actor for OverlayListenerActor {
    type Msg = AstroProtocol;

    fn receive(
        &mut self,
        _ctx: &Context<Self::Msg>,
        _msg: Self::Msg,
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
                    let peer = Box::new(Peer::new(stream, CONFIG.local_node().address()));
                    overlay_manager_ref(ctx).tell(AstroProtocol::HandleOverlayIncomingPeerCmd(peer), None)
                }
                Err(e) => {
                    info!("[Overlay][Listener] connection failed, cause: {:?}", e);
                }
            }
        }
        unreachable!();
    }
}
