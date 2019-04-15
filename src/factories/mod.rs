#![allow(dead_code)]

pub mod external_xdr;
pub mod external_xdr_stub;
pub mod flood_gate;
pub mod internal_xdr;
pub mod local_node;
pub mod peer;

use serde::ser::Serialize;

pub fn prepare<T: Serialize>(rust_struct: T, ruby_struct: String) -> (Vec<u8>, Vec<u8>) {
    let rust_xdr = internal_xdr::serialize(rust_struct);

    let ruby_xdr = external_xdr::serialize(ruby_struct);
    (rust_xdr, ruby_xdr)
}

pub fn prepare_with_stub<T: Serialize>(rust_struct: T, stub_struct: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let rust_xdr = internal_xdr::serialize(rust_struct);

    (rust_xdr, stub_struct)
}
