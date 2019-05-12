#[macro_use]
extern crate diesel;

mod astro_protocol;
mod config;
mod crypto;
mod database;
mod factories;
mod network;
mod overlay;
mod schema;
mod scp;
mod xdr;

use astro_protocol::AstroProtocol;
use overlay::overlay_manager::OverlayManagerActor;
use riker::actors::*;
use riker_default::DefaultModel;

fn main() {
    env_logger::init();
    database::init();

    let model: DefaultModel<AstroProtocol> = DefaultModel::new();
    let sys = ActorSystem::new(&model).unwrap();
    let props = OverlayManagerActor::props();
    sys.actor_of(props, "overlay_manager").unwrap();

    loop {}
}
