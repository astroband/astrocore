# WORK IN PROGRESS
This crate try to implement stellar node on rust. Still in very ealy stage.

## Current stage
- [x] Cryptography (mod crypto)
- [x] LocalNode representation (mod scp)
- [x] Handsnake process (peer.rs)
- [x] XDR communication (xdr.rs)
- [x] Working with Database (SQLite)
- [x] Accepting new connections (Overlay Listener)
- [x] Retranslate messages (Overlay FloodGate)
- [x] Finding new nodes (Overlay Manager)
- [ ] Represent current Ledger (work in progress)

## Requirements
SQLite
libsodium-dev

## Quick start

1. Clone repo
2. Change `.env.example` to `.env` and change DATABASE_URL
3. Install diesel and run migrations:
```
cargo install diesel_cli --no-default-features --features sqlite

diesel migration run
```
4. Run astrocore:
```
RUST_LOG="astrocore=info" RUSTFLAGS=-Awarnings cargo run
```
