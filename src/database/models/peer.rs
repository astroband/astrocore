use super::{db_conn, schema::peers, CONFIG};
use chrono::NaiveDateTime;
use diesel::prelude::*;

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
        let new_peer = NewPeer { ip, port, nextattempt: diesel::dsl::now };
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
            let new_peer = NewPeer {
                ip: initial_peer.ip(),
                port: *initial_peer.port() as i32,
                nextattempt: diesel::dsl::now,
            };

            diesel::insert_into(peers::table)
                .values(&new_peer)
                .execute(&*db_conn());
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
