pub mod models;
pub mod queries;
pub mod schema;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use once_cell::sync::OnceCell;

pub static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();

pub fn get_conn() -> PooledConnection<ConnectionManager<PgConnection>> {
    DB.get()
        .expect("No database connection set")
        .get()
        .expect("Failed to get database connection")
}
