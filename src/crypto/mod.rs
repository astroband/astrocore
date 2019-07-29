#![allow(dead_code)]

use sodiumoxide;

mod error;
mod keypair;
mod signature;
mod strkey;

// pub use self::ecdh::{Curve25519Public, Curve25519Secret};
pub use self::keypair::from_secret_seed;

// Initialize the sodium library and chooses faster version of the primitives
// if possible.
//
// `init` also makes `KeyPair::random()` thread-safe.
pub fn init() {
    sodiumoxide::init().unwrap();
}
