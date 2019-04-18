use super::{
    dotenv, env, info, lazy_static, ConnectionManager, PgConnection, Pool, PooledConnection, CONFIG,
};

lazy_static! {
    pub static ref DB: Pool<ConnectionManager<PgConnection>> = establish_connection();
}

fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("[DB] DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url.to_owned());
    let pool = Pool::builder()
        .max_size(*CONFIG.db_pool())
        .build(manager)
        .expect(&format!(
            "[DB] Failed to create pool. Check your db settings"
        ));

    info!("[DB] Connection to database established");

    pool
}

pub(crate) fn db_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    DB.get().unwrap()
}
