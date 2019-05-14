#![allow(dead_code)]

use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use toml;

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = Config::init();
}

#[derive(Debug, Deserialize)]
pub struct Config {
    network: String,
    local_node: LocalNode,
    initial_peers: Vec<InitialPeer>,
    public_passphrase: String,
    test_passphrase: String,
    seed: String,
    db_pool: u32,
    // Maximum of connected peers
    max_peers: u32,
    min_peers: u32,
}

impl Config {
    pub fn init() -> Config {
        let mut file = File::open(Path::new("config.toml")).unwrap();
        let mut toml_str = String::new();
        file.read_to_string(&mut toml_str).unwrap();
        toml::from_str::<Config>(&toml_str).unwrap()
    }

    pub fn local_node(&self) -> &LocalNode {
        &self.local_node
    }

    pub fn initial_peers(&self) -> &Vec<InitialPeer> {
        &self.initial_peers
    }

    pub fn network(&self) -> &String {
        &self.network
    }

    pub fn seed(&self) -> &String {
        &self.seed
    }

    pub fn test_passphrase(&self) -> &String {
        &self.test_passphrase
    }

    pub fn public_passphrase(&self) -> &String {
        &self.public_passphrase
    }

    pub fn db_pool(&self) -> &u32 {
        &self.db_pool
    }

    pub fn max_peers(&self) -> &u32 {
        &self.max_peers
    }

    pub fn min_peers(&self) -> &u32 {
        &self.min_peers
    }
}

#[derive(Debug, Deserialize)]
pub struct LocalNode {
    ip: String,
    port: u64,
}

impl LocalNode {
    pub fn ip(&self) -> &String {
        &self.ip
    }

    pub fn port(&self) -> &u64 {
        &self.port
    }

    pub fn address(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct InitialPeer {
    ip: String,
    port: u64,
}

impl InitialPeer {
    pub fn ip(&self) -> &String {
        &self.ip
    }

    pub fn port(&self) -> &u64 {
        &self.port
    }

    pub fn address(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }
}
