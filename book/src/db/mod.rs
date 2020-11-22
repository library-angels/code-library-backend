use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub mod models;
pub mod queries;
pub mod schema;

pub fn get_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    crate::DB
        .get()
        .expect("No database connection set")
        .get()
        .expect("Failed to get database connection")
}
