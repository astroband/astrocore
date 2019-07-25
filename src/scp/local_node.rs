use super::{crypto, lazy_static, xdr, Network, CONFIG};
use ed25519_dalek::Keypair;

lazy_static! {
    #[derive(Debug)]
    pub static ref LOCAL_NODE: LocalNode = LocalNode::new(
        CONFIG.seed().to_owned(),
        &Network::network().network_id(),
    );
}

#[derive(Debug)]
pub struct LocalNode {
    /// Secret seed in our node for build keys
    pub secret_seed: String,
    /// Key pair
    pub key_pair: Keypair,
    /// Hash for used network
    pub network_id: xdr::Hash,
}

impl LocalNode {
    /// Return Node instance
    pub fn new(secret_seed: String, stellar_network: &[u8]) -> LocalNode {
        let key_pair = crypto::from_secret_seed(&secret_seed).unwrap();

        let mut network_id: [u8; 32] = Default::default();
        network_id.copy_from_slice(stellar_network);

        LocalNode {
            secret_seed,
            key_pair,
            network_id: xdr::Hash(network_id),
        }
    }

    pub fn network_id(&self) -> &xdr::Hash {
        &self.network_id
    }

    pub fn secret_seed(&self) -> &String {
        &self.secret_seed
    }

    pub fn key_pair(&self) -> &ed25519_dalek::Keypair {
        &self.key_pair
    }
}
