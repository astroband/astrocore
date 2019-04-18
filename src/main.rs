#[macro_use]
extern crate diesel;

mod config;
mod crypto;
mod database;
mod factories;
mod network;
mod overlay;
mod schema;
mod scp;
mod xdr;
use overlay::overlay_manager::OverlayManager;

fn main() {
    env_logger::init();
    OverlayManager::new().start();
}
