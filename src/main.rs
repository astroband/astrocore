#[macro_use]
extern crate diesel;

mod actors;
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

fn main() {
    env_logger::init();
    database::init();
    actors::start();
    loop {}
}
