pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn db_pool(database_url: &str) -> DbPool {
    Pool::new(ConnectionManager::new(database_url)).expect("Failed to create database pool")
}
