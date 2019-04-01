use log::{error, info};
use std::net::TcpStream;
use std::time::Duration;

mod crypto;
mod network;
mod peer;
mod xdr;
use peer::Peer;
mod node_info;
use node_info::NodeInfo;

const SECRET_TEST_SEED: &str = "SATKBWSRLPHGM2FKMLZ4QNH64XYKP7J2O6U5QNFBJHYEXDSHN75R5MVE";

fn main() {
    env_logger::init();

    let seed = String::from(SECRET_TEST_SEED);
    let node = NodeInfo::new(seed, &network::Network::test_network().network_id());
    let peer_address = String::from("54.160.175.7:11625"); // test address

    match TcpStream::connect_timeout(&peer_address.parse().unwrap(), Duration::new(5, 0)) {
        Ok(stream) => {
            info!("Successfully connected to peer {}", peer_address);
            let cloned_stream = stream.try_clone().expect("clone failed...");

            let mut peer = Peer::new(&node, cloned_stream, peer_address);
            peer.start_authentication();

            loop {
                let message_content = peer.receive_message();
                match message_content {
                    Ok(msg) => info!("\n{:?}", msg),
                    Err(e) => error!("Cant read XDR message cause: {}", e),
                };
            }
        }
        Err(e) => {
            error!("Failed to connect: {}", e);
        }
    }
    info!("Terminated.");
}
