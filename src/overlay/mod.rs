#![allow(dead_code, unused_must_use)]

use sha2::digest::Digest;
use sha2::Sha256;

pub(crate) mod flood_gate;
pub(crate) mod overlay_manager;
pub(crate) mod peer;

pub(crate) use crate::{
    config::CONFIG,
    database,
    scp::local_node::{LocalNode, LOCAL_NODE},
    xdr,
};
pub(crate) use byteorder::{BigEndian, WriteBytesExt};
pub(crate) use itertools;
pub(crate) use log::{debug, error, info};
pub(crate) use rand::Rng;
pub(crate) use serde_xdr;
pub(crate) use sha2;

pub(crate) use self::flood_gate::FloodGate;
pub(crate) use self::overlay_manager::OverlayManager;
pub(crate) use self::peer::{Peer, PeerInterface};

pub(crate) fn message_abbr(message: &xdr::StellarMessage) -> String {
    let bytes = serde_xdr::to_bytes(message).unwrap();
    String::from_utf8_lossy(Sha256::digest(&bytes.as_slice()).as_slice()).to_string()
}
