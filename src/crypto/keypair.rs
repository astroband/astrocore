use super::error::{Error, Result};
use super::strkey;
use ed25519_dalek::{Keypair, PublicKey, SecretKey};

pub fn from_secret_seed(data: &str) -> Result<Keypair> {
    let bytes = strkey::decode_secret_seed(&data)?;
    let secret = SecretKey::from_bytes(&bytes).or(Err(Error::InvalidSeed))?;
    let public = PublicKey::from(&secret);
    Ok(Keypair {
        secret,
        public
    })
}
