use crate::crypto;
use crate::xdr;

pub struct LocalNode {
    /// Secret seed in our node for build keys
    pub secret_seed: String,
    /// Key pair
    pub key_pair: crypto::KeyPair,
    /// Hash for used network
    pub network_id: xdr::Hash,
}

impl LocalNode {
    /// Return Node instance
    pub fn new(seed: String, stellar_network: &Vec<u8>) -> LocalNode {
        let key_pair = crypto::KeyPair::from_secret_seed(&seed).unwrap();

        let mut network_id: [u8; 32] = Default::default();
        network_id.copy_from_slice(&stellar_network[..]);

        LocalNode {
            secret_seed: seed,
            key_pair: key_pair,
            network_id: xdr::Hash(network_id),
        }
    }
}
