#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate xdrgen;

fn main() {
    xdrgen::compile("src/xdr/Stellar-types.x").unwrap();
    xdrgen::compile("src/xdr/Stellar-ledger-entries.x").unwrap();
    xdrgen::compile("src/xdr/Stellar-overlay.x").unwrap();
    xdrgen::compile("src/xdr/Stellar-ledger.x").unwrap();
    xdrgen::compile("src/xdr/Stellar-SCP.x").unwrap();
    xdrgen::compile("src/xdr/Stellar-transaction.x").unwrap();
}
