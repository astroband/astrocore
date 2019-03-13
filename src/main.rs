extern crate xdr_codec;
extern crate xdrgen;

use log::{error, info};
use std::net::TcpStream;
use std::time::Duration;

mod crypto;
mod network;
mod peer;
use peer::Peer;
mod node_info;
use node_info::NodeInfo;

mod xdr {
    use xdr_codec;

    // NOTE: Return live-generated files, when xdrgen was completed
    // for now just store generated files in repo

    // include!(concat!(env!("OUT_DIR"), "/Stellar_types_xdr.rs"));
    // include!(concat!(env!("OUT_DIR"), "/Stellar_SCP_xdr.rs"));
    // include!(concat!(env!("OUT_DIR"), "/Stellar_ledger_entries_xdr.rs"));
    // include!(concat!(env!("OUT_DIR"), "/Stellar_transaction_xdr.rs"));
    // include!(concat!(env!("OUT_DIR"), "/Stellar_ledger_xdr.rs"));
    // include!(concat!(env!("OUT_DIR"), "/Stellar_overlay_xdr.rs"));

    include!("generated/Stellar_types_xdr.rs");
    include!("generated/Stellar_SCP_xdr.rs");
    include!("generated/Stellar_ledger_entries_xdr.rs");
    include!("generated/Stellar_transaction_xdr.rs");
    include!("generated/Stellar_ledger_xdr.rs");
    include!("generated/Stellar_overlay_xdr.rs");
}

const SECRET_TEST_SEED: &str = "SATKBWSRLPHGM2FKMLZ4QNH64XYKP7J2O6U5QNFBJHYEXDSHN75R5MVE";

fn main() {
    env_logger::init();

    let seed = String::from(SECRET_TEST_SEED);
    let node = NodeInfo::new(seed, &network::Network::test_network().network_id());
    let peer_address = String::from("54.160.175.7:11625"); // test address

    match TcpStream::connect_timeout(&peer_address.parse().unwrap(), Duration::new(5, 0)) {
        Ok(mut stream) => {
            info!("Successfully connected to peer {}", peer_address);
            let cloned_stream = stream.try_clone().expect("clone failed...");

            let mut peer = Peer::new(&node, cloned_stream, peer_address);
            peer.start_authentication();

            loop {
                let message_content = peer.receive_message();
                info!("\n{:?}", message_content.V0.message);
            }
        }
        Err(e) => {
            error!("Failed to connect: {}", e);
        }
    }
    info!("Terminated.");
}
