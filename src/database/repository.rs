use crate::config::CONFIG;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use lazy_static::lazy_static;
use log::info;

fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(CONFIG.database_url().to_owned());
    let pool = Pool::builder().build(manager).expect(&format!(
        "Failed to create pool to {}",
        CONFIG.database_url()
    ));

    info!(
        "Connection to database established {}",
        CONFIG.database_url()
    );

    pool
}

lazy_static! {
    pub static ref DB: Pool<ConnectionManager<PgConnection>> = establish_connection();
}
