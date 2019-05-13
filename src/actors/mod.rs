#![allow(dead_code, unused_must_use)]
#![allow(clippy::new_ret_no_self)]

mod flood_gate;
mod overlay_manager;
mod peer;

pub(crate) use crate::{
    astro_protocol::AstroProtocol,
    config::CONFIG,
    overlay::{message_abbr, FloodGate, OverlayManager, Peer, PeerInterface},
    xdr,
};
pub(crate) use log::{debug, info};
pub(crate) use riker;

use self::flood_gate::FloodGateActor;
pub(crate) use self::overlay_manager::OverlayManagerActor;
use self::peer::PeerActor;

fn address_peer_to_actor(address: String) -> String {
    address.replace(".", "-").replace(":", "-")
}

pub(crate) fn start() {
    use riker::actors::*;
    use riker_default::DefaultModel;

    let model: DefaultModel<AstroProtocol> = DefaultModel::new();
    let sys = ActorSystem::new(&model).unwrap();
    let props = OverlayManagerActor::props();

    sys.actor_of(props, "overlay_manager").unwrap();
}
