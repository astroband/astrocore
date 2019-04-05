pub mod flood_gate;
pub mod peer;

use crate::crypto;
use crate::xdr;
use serde_xdr;

#[macro_use]
use double;

pub fn message_abbr(message: &xdr::StellarMessage) -> String {
    let bytes = serde_xdr::to_bytes(message).unwrap();
    String::from_utf8_lossy(crypto::hash(&bytes.as_slice()).as_slice()).to_string()
}
