# WORK IN PROGRESS
This crate try to implement stellar node on rust. Still in very ealy stage.

## Current stage
We already complete handsnake process with remote peer. So you can monitor all messages from remote peer or explore the stellar network jumping between known addresses.

## Quick start

1. Clone repo
2. Run command:
```
RUST_LOG="astrocore=debug" RUSTFLAGS=-Awarnings cargo run
```

## Current problems

Many problems with XDR library. See the [issues](https://github.com/Arkweid/stellar-explorer-rust/issues).

## Tests

1. go to `ruby_xdr/`
2. Run
```
bundle install
```
3. Return to root directory
4. Run
```
RUSTFLAGS=-Awarnings cargo test --test serialize_integration_test
```
