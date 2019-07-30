#![allow(dead_code)]

mod error;
mod keypair;
mod strkey;

pub use self::keypair::from_secret_seed;
