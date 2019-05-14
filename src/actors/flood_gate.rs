use super::{
    peer_ref, debug, message_abbr, riker::actors::*, xdr, AstroProtocol,
    FloodGate,
};
use std::collections::HashSet;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub(crate) struct FloodGateActor {
    state: FloodGate,
}

impl FloodGateActor {
    pub fn new() -> BoxActor<AstroProtocol> {
        let actor = FloodGateActor {
            state: FloodGate::new(),
        };

        Box::new(actor)
    }

    pub fn props() -> BoxActorProd<AstroProtocol> {
        Props::new(Box::new(FloodGateActor::new))
    }

    /// Send message to anyone you haven't gotten it from
    pub fn broadcast(
        &mut self,
        ctx: &Context<AstroProtocol>,
        message: xdr::StellarMessage,
        force: bool,
        peers: HashSet<String>,
    ) {
        if self.state.m_shutting_down {
            return;
        };

        let index = message_abbr(&message);

        // no one has sent us this message
        if self.state.flood_map.get(&index).is_none() || force {
            self.state
                .add_record(&message, "self".to_string(), unix_time());
        };

        let record = match self.state.flood_map.get_mut(&index) {
            Some(rec) => rec,
            None => return,
        };

        let previous_sent = record.m_peers_told.clone();
        for peer in peers {
            peer_ref(&peer, ctx).tell(AstroProtocol::SendPeerMessageCmd(message.to_owned()), None);
            record.m_peers_told.push(peer.to_owned());
        }

        debug!(
            "[Overlay][FloodGate] broadcast told {}",
            record.m_peers_told.len() - previous_sent.len()
        );
    }
}

impl Actor for FloodGateActor {
    type Msg = AstroProtocol;

    fn receive(
        &mut self,
        ctx: &Context<Self::Msg>,
        msg: Self::Msg,
        _sender: Option<ActorRef<Self::Msg>>,
    ) {
        debug!("FLOOD_GATE RECEIVE: {:?}", msg);
        match msg {
            AstroProtocol::AddRecordFloodGateCmd(message, address, seq_ledger) => {
                self.state.add_record(&message, address, seq_ledger);
            }
            AstroProtocol::BroadcastFloodGateCmd(message, force, peers) => {
                self.broadcast(ctx, message, force, peers);
            }
            AstroProtocol::ClearFloodGateCmd(seq_ledger) => self.state.clear_below(seq_ledger),
            _ => unreachable!(),
        }
    }
}

// stub for current ledger value
fn unix_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}
