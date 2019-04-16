# WORK IN PROGRESS
This crate try to implement stellar node on rust. Still in very ealy stage.

## Current stage
We already complete handsnake process with remote peer. So you can monitor all messages from remote peer or explore the stellar network jumping between known addresses.

## Requirements
PostgreSQL

## Quick start

1. Clone repo
2. Change `.env.example` to `.env` and change DATABASE_URL
3. Install diesel and run migrations:
```
cargo install diesel_cli --no-default-features --features postgres

diesel migration run
```
4. Run astrocore:
```
RUST_LOG="astrocore=info" RUSTFLAGS=-Awarnings cargo run
```
