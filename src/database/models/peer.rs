use super::{db_conn, schema::peers, CONFIG};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use log::warn;
use std::net::ToSocketAddrs;

#[derive(Queryable, Debug)]
pub struct Peer {
    pub ip: String,
    pub port: i32,
    pub nextattempt: NaiveDateTime,
    pub numfailures: i32,
}

type Result<T> = std::result::Result<T, diesel::result::Error>;

impl Peer {
    pub fn all() -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers.load::<Peer>(&*db_conn())
    }

    pub fn create(ip: &str, port: i32) -> Result<usize> {
        let new_peer = NewPeer {
            ip,
            port,
            nextattempt: diesel::dsl::now,
        };
        diesel::insert_into(peers::table)
            .values(&new_peer)
            .execute(&*db_conn())
    }

    pub fn get(g_ip: &str, g_port: i32) -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers
            .filter(ip.eq(g_ip))
            .filter(port.eq(g_port))
            .load::<Peer>(&*db_conn())
    }

    pub fn delete(g_ip: &str, g_port: i32) -> Result<usize> {
        use self::peers::dsl::*;

        diesel::delete(peers.filter(ip.eq(g_ip)).filter(port.eq(g_port))).execute(&*db_conn())
    }

    pub fn load_initial_peers() {
        for initial_peer in CONFIG.initial_peers() {
            // perform DNS lookup, if necessary
            match initial_peer.address().to_socket_addrs() {
                Ok(mut addrs) => {
                    if let Some(addr) = addrs.next() {
                        let new_peer = NewPeer {
                            ip: &addr.ip().to_string(),
                            port: addr.port() as i32,
                            nextattempt: diesel::dsl::now,
                        };

                        diesel::insert_into(peers::table)
                            .values(&new_peer)
                            .execute(&*db_conn());
                    }
                }
                Err(e) => {
                    warn!(
                        "DNS lookup for the peer {} failed: {}",
                        initial_peer.address(),
                        e
                    );
                    continue;
                }
            }
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", &self.ip, &self.port)
    }
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewPeer<'a> {
    pub ip: &'a str,
    pub port: i32,
    pub nextattempt: diesel::dsl::now,
}
