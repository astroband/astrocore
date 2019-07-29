# WORK IN PROGRESS
This crate try to implement stellar node on rust. Still in very ealy stage.

<a href="https://evilmartians.com/?utm_source=astrocore">
<img src="https://evilmartians.com/badges/sponsored-by-evil-martians.svg" alt="Sponsored by Evil Martians" width="236" height="54"></a>

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
