pub(crate) mod flood_gate;
pub(crate) mod overlay_manager;
pub(crate) mod peer;

pub(crate) use crate::{
    config::CONFIG,
    crypto, database,
    scp::local_node::{LocalNode, LOCAL_NODE},
    xdr,
};
pub(crate) use byteorder::{BigEndian, WriteBytesExt};
pub(crate) use itertools;
pub(crate) use log::{debug, error, info};
pub(crate) use rand::Rng;
pub(crate) use serde_xdr;
pub(crate) use sha2;

pub fn message_abbr(message: &xdr::StellarMessage) -> String {
    let bytes = serde_xdr::to_bytes(message).unwrap();
    String::from_utf8_lossy(crypto::hash(&bytes.as_slice()).as_slice()).to_string()
}
