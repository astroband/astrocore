extern crate xdr_codec;
extern crate xdrgen;

use std::io::Read;
use std::net::TcpStream;
use std::str::from_utf8;
use std::time::Duration;

mod crypto;
mod network;
mod peer;
use peer::Peer;
mod node_info;
use node_info::NodeInfo;

mod xdr {
    use xdr_codec;

    include!(concat!(env!("OUT_DIR"), "/Stellar_types_xdr.rs"));
    include!(concat!(env!("OUT_DIR"), "/Stellar_SCP_xdr.rs"));
    include!(concat!(env!("OUT_DIR"), "/Stellar_ledger_entries_xdr.rs"));
    include!(concat!(env!("OUT_DIR"), "/Stellar_transaction_xdr.rs"));
    include!(concat!(env!("OUT_DIR"), "/Stellar_ledger_xdr.rs"));
    include!(concat!(env!("OUT_DIR"), "/Stellar_overlay_xdr.rs"));
}

const SECRET_TEST_SEED: &str = "SATKBWSRLPHGM2FKMLZ4QNH64XYKP7J2O6U5QNFBJHYEXDSHN75R5MVE";

fn main() {
    let seed = String::from(SECRET_TEST_SEED);
    let node = NodeInfo::new_test(seed);
    let peer_address = String::from("0.0.0.0:3000");

    match TcpStream::connect_timeout(&peer_address.parse().unwrap(), Duration::new(3, 0)) {
        Ok(mut stream) => {
            println!("Successfully connected to peer {}", peer_address);
            let cloned_stream = stream.try_clone().expect("clone failed...");

            let mut peer = Peer::new(&node, cloned_stream, peer_address);
            peer.start_authentication();

            let mut data: Vec<u8> = Default::default();
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("reply: {}", text);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
