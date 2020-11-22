pub mod models;
pub mod queries;
pub mod schema;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub fn get_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    crate::DB
        .get()
        .expect("No database connection set")
        .get()
        .expect("Failed to get database connection")
}
