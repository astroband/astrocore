use crate::crypto;
use crate::xdr;
use crate::network;

pub struct NodeInfo {
    /// Secret seed in our node for build keys
    pub secret_seed: String,
    /// Key pair
    pub key_pair: crypto::KeyPair,
    /// Hash for used network
    /// NOTE: TEST NETWORK HARDCODED
    pub network_id: xdr::Hash,
}

impl NodeInfo {
    /// Return Node instance
    /// NOTE: TEST NETWORK HARDCODED
    pub fn new_test(seed: String) -> NodeInfo {
        let key_pair = crypto::KeyPair::from_secret_seed(&seed).unwrap();

        let mut network_id: [u8; 32i64 as usize] = Default::default();
        network_id.copy_from_slice(&network::Network::test_network().network_id()[..]);

        NodeInfo {
            secret_seed: seed.clone(),
            key_pair: key_pair,
            network_id: xdr::Hash { 0: network_id },
        }
    }
}
