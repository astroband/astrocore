use sodiumoxide::crypto::sign::ed25519;

use crate::crypto;
use crate::xdr;
use crate::network;

pub struct NodeInfo {
    /// Secret seed in our node for build keys
    pub secret_seed: String,
    /// Public key
    pub public_key: xdr::uint256,
    /// Secret key
    pub private_key: ed25519::SecretKey,
    /// Hash for used network
    /// NOTE: TEST NETWORK HARDCODED
    pub network_id: xdr::Hash,
}

impl NodeInfo {
    /// Return Node instance
    /// NOTE: TEST NETWORK HARDCODED
    pub fn new_test(seed: String) -> NodeInfo {
        let key_pair = crypto::KeyPair::from_secret_seed(&seed).unwrap();
        key_pair.secret_key().inner();

        let mut public_key: [u8; 32] = Default::default();
        public_key.copy_from_slice(key_pair.public_key().buf());

        let mut network_id: [u8; 32i64 as usize] = Default::default();
        network_id.copy_from_slice(&network::Network::test_network().network_id()[..]);

        NodeInfo {
            secret_seed: seed.clone(),
            public_key: xdr::uint256 { 0: public_key },
            private_key: key_pair.secret_key().inner().clone(),
            network_id: xdr::Hash { 0: network_id },
        }
    }
}
