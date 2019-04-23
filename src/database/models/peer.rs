use super::{db_conn, schema::peers, CONFIG};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Peer {
    pub id: i32,
    pub address: String,
}

type Result<T> = std::result::Result<T, diesel::result::Error>;

impl Peer {
    pub fn all<'a>() -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers.load::<Peer>(&*db_conn())
    }

    pub fn create<'a>(addr: &String) -> Result<Peer> {
        let new_peer = NewPeer { address: addr };

        diesel::insert_into(peers::table)
            .values(&new_peer)
            .get_result(&*db_conn())
    }

    pub fn get<'a>(addr: &String) -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers.filter(address.eq(addr)).load::<Peer>(&*db_conn())
    }

    pub fn delete<'a>(addr: &String) -> Result<usize> {
        use self::peers::dsl::*;

        diesel::delete(peers.filter(address.eq(addr))).execute(&*db_conn())
    }

    pub fn load_initial_peers() {
        for initial_peer in CONFIG.initial_peers() {
            let new_peer = NewPeer {
                address: &initial_peer.address(),
            };
            diesel::insert_into(peers::table)
                .values(&new_peer)
                .execute(&*db_conn());
        }
    }

    pub fn address(&self) -> &String {
        &self.address
    }
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewPeer<'a> {
    pub address: &'a String,
}
