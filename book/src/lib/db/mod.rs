pub(crate) mod models;
pub(crate) mod queries;
pub(crate) mod schema;

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn new_db_pool(database_url: &str) -> DbPool {
    Pool::new(ConnectionManager::new(database_url))
        .expect("Failed creating new database connection")
}
