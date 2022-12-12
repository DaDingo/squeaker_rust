use diesel::{pg::PgConnection, r2d2::ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create connection pool.")
}
