Astrocore aims to become an alternative implementation of the stellar-core, the core component of the Stellar network. This project is still in the very early development stage

<a href="https://evilmartians.com/?utm_source=astrocore">
<img src="https://evilmartians.com/badges/sponsored-by-evil-martians.svg" alt="Sponsored by Evil Martians" width="236" height="54"></a>

# Quickstart

## Requirements

`rust >= 1.33.0 (nightly)` and `sqlite3`

## Building


1. clone this repo
1. move `.env.example` to `.env` and set `DATABASE_URL` variable there
1. install `diesel` and run migrations:
    ```Shell
    cargo install diesel_cli --no-default-features --features sqlite
    diesel migration run
    ```

## Running

    RUST_LOG="astrocore=info" RUSTFLAGS=-Awarnings cargo run
    
# Why another implementation?

Stellar itself is a blockchain, decentralized by nature. Having more than one core node implementation is right for decentralization. Also, reimplementing can help to discover possible bugs in the current codebase.

Reference implementation doesn't have any specification of how it works, so we are going to use the reverse engineering approach and kickstart specification writing process(check out [Github wiki](https://github.com/astroband/astrocore/wiki)). Having one facilitates the creation of other implementations tremendously. We put it in the .

## Why Rust?

The Rust language seems perfect for this task because it focuses heavily on performance and reliability. Moreover, Rust code is developer-friendly, and this can help to increase the number of potential contributors.

# Roadmap
## Current stage

For starters, we took up networking part. At the moment Astrocore is able to connect to the existing Stellar network, discover and authenticate peers, and listen to the incoming messages, doing nothing in response


## Next Steps

`Stellar-core` consists of a few major components, according to the docs:


* SCP
* Herder
* Overlay
* Ledger
* History
* BucketList
* Transactions

We are going to reimplement them and their functionality consecutively. We started from the "Overlay" component, and you can find more in the "Current stage" section. We don't have reliable estimations on the time of delivery of any of these components, because Astrocore is still a very experimental project.

# Astrocore team

[@Arkweid](https://github.com/Arkweid)<br/>
[@nebolsin](https://github.com/nebolsin)<br/>
[@charlie-wasp](https://github.com/charlie-wasp)<br/>
