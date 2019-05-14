use super::{
    peer_actor_name, info, riker::actors::*, xdr, AstroProtocol, FloodGateActor,
    OverlayManager, Peer, PeerActor, CONFIG, overlay_manager_ref, flood_gate_ref
};
use std::net::TcpListener;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
        if self.state.reached_min_of_authenticated_peers() {
            return;
        }

        let limit = self.state.peers_to_authenticated_min_limit() as usize;
        let taked_peers: Vec<_> = self
            .state
            .known_peer_adresses()
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

    pub fn handle_new_incoming_peer(&mut self, ctx: &Context<AstroProtocol>, peer: Peer) {
        if self.state.reached_max_of_authenticated_peers() {
            info!("[Overlay][Listener] new incoming peer {} dropped, cause: limit of peers", peer.peer_addr());
        } else {
            info!("[Overlay][Listener] new incoming peer {}", peer.peer_addr());
            let name = peer_actor_name(&peer.peer_addr());
            ctx.system
                .actor_of(PeerActor::incoming_peer_props(peer), &name);
        }
    }

    pub fn handle_new_initiated_peer(&mut self, ctx: &Context<AstroProtocol>, address: String) {
        let name = peer_actor_name(&address);
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
            }
            xdr::StellarMessage::Transaction(_) | xdr::StellarMessage::Envelope(_) => {
                let flood_gate = flood_gate_ref(ctx);
                flood_gate.tell(
                    AstroProtocol::AddRecordFloodGateCmd(message.to_owned(), address, unix_time()),
                    None,
                );
                flood_gate.tell(
                    AstroProtocol::BroadcastFloodGateCmd(
                        message.to_owned(),
                        false,
                        self.state.authenticated_peers().clone(),
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

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        self.state = OverlayManager::new();
        self.state.populate_known_peers_from_db();
    }

    fn receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        sender: Option<ActorRef<Self::Msg>>,
    ) {
        match msg {
            AstroProtocol::CheckOverlayMinConnectionsCmd => self.check_min_connections(ctx),
            AstroProtocol::HandleOverlayIncomingPeerCmd(peer) => {
                self.handle_new_incoming_peer(ctx, *peer)
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

// stub for current ledger value
fn unix_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
