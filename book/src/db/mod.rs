use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub mod models;
pub mod queries;
pub mod schema;

pub fn get_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    match crate::DB.get() {
        Some(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(e) => panic!("Failed to get database connection: {}", e),
        },
        None => panic!("No database connection set"),
    }
}
