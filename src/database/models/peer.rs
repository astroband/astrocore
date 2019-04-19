use super::{db_conn, schema::peers};
use diesel::prelude::*;

#[derive(Queryable, Debug)]
pub struct Peer {
    pub id: i32,
    pub address: String,
}

type Result<T> = std::result::Result<T, diesel::result::Error>;

impl Peer {
    pub fn create_peer<'a>(addr: &String) -> Result<Peer> {
        let new_peer = NewPeer { address: addr };

        diesel::insert_into(peers::table)
            .values(&new_peer)
            .get_result(&*db_conn())
    }

    pub fn get_peer<'a>(addr: &String) -> Result<Vec<Peer>> {
        use self::peers::dsl::*;

        peers.filter(address.eq(addr)).load::<Peer>(&*db_conn())
    }

    pub fn delete_peer<'a>(addr: &String) -> Result<usize> {
        use self::peers::dsl::*;

        diesel::delete(peers.filter(address.eq(addr))).execute(&*db_conn())
    }
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewPeer<'a> {
    pub address: &'a String,
}
