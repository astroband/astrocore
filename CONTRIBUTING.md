# Contributing
Thanks for taking the time to contribute! Feel free to make Pull Request with you changes.

# Requirements

Rust >= 1.33.0

# Proccess

1. Fork repo
2. git clone <forked_repo>
3. Change `.env.example` to `.env` and change DATABASE_URL
3. Install diesel and run migrations:
```
sudo apt-get install sqlite3 libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite

diesel migration run
```
5. Run astrocore:
```bash
RUST_LOG="astrocore=debug" RUSTFLAGS=-Awarnings cargo run
```
6. Make changes
7. Push your changes in repo
