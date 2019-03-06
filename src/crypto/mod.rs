#![allow(dead_code)]

use sodiumoxide;
use sodiumoxide::crypto::hash::sha256;
use sodiumoxide::randombytes;

mod ecdh;
mod error;
mod keypair;
mod sha;
mod signature;
mod strkey;

pub use self::ecdh::{Curve25519Public, Curve25519Secret};
pub use self::keypair::{KeyPair, PublicKey, SecretKey};
pub use self::sha::{HmacSha256Key, HmacSha256Mac};

/// Compute sha256 hash of `m`.
pub fn hash(m: &[u8]) -> Vec<u8> {
    let digest = sha256::hash(&m);
    digest.0.to_vec()
}

/// Generate `size` random bytes.
pub fn random_bytes(size: usize) -> Vec<u8> {
    randombytes::randombytes(size)
}

// Initialize the sodium library and chooses faster version of the primitives
// if possible.
//
// `init` also makes `KeyPair::random()` thread-safe.
pub fn init() -> () {
    sodiumoxide::init().unwrap();
}
