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
pub(crate) use log::{debug, info, error};
pub(crate) use riker;

pub(crate) use self::overlay_manager::OverlayManagerActor;
use self::flood_gate::FloodGateActor;
use self::peer::PeerActor;

use riker::actors::*;
use riker_default::DefaultModel;

pub(crate) fn start() {
    let model: DefaultModel<AstroProtocol> = DefaultModel::new();
    let sys = ActorSystem::new(&model).unwrap();
    let props = OverlayManagerActor::props();

    sys.actor_of(props, "overlay_manager").unwrap();
}

fn overlay_manager_ref(ctx: &Context<AstroProtocol>) -> ActorSelection<AstroProtocol> {
    ctx.select("/user/overlay_manager").unwrap()
}

fn flood_gate_ref(ctx: &Context<AstroProtocol>) -> ActorSelection<AstroProtocol> {
    ctx.select("/user/flood_gate").unwrap()
}

fn peer_ref(address: &str, ctx: &Context<AstroProtocol>) -> ActorSelection<AstroProtocol> {
    ctx.select(&format!("/user/peer-{}", peer_actor_name(address))).unwrap()
}

fn peer_actor_name(address: &str) -> String {
    format!("peer-{}", address.replace(".", "-").replace(":", "-"))
}
