use crate::factories::{external_xdr, internal_xdr};
use serde::ser::Serialize;

pub fn prepare<T: Serialize>(rust_struct: T, ruby_struct: String) -> (Vec<u8>, Vec<u8>) {
    let rust_xdr = internal_xdr::serialize(rust_struct);

    let ruby_xdr = external_xdr::serialize(ruby_struct);
    (rust_xdr, ruby_xdr)
}
