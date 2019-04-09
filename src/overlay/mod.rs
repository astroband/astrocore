pub mod flood_gate;
pub mod overlay_manager;
pub mod peer;

use crate::crypto;
use crate::xdr;
use serde_xdr;

pub fn message_abbr(message: &xdr::StellarMessage) -> String {
    let bytes = serde_xdr::to_bytes(message).unwrap();
    String::from_utf8_lossy(crypto::hash(&bytes.as_slice()).as_slice()).to_string()
}
