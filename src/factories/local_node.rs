use crate::network;
use crate::scp::local_node::*;

const SECRET_TEST_SEED: &str = "SATKBWSRLPHGM2FKMLZ4QNH64XYKP7J2O6U5QNFBJHYEXDSHN75R5MVE";

pub fn build_local_node() -> LocalNode {
    let seed = String::from(SECRET_TEST_SEED);
    LocalNode::new(seed, &network::Network::test_network().network_id())
}
