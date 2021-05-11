pub(crate) mod models;
pub(crate) mod queries;
pub(crate) mod schema;

use sqlx::{pool::PoolConnection, Error, PgPool, Pool, Postgres};

pub type DbPool = PgPool;
pub type DbConnection = PoolConnection<Postgres>;
pub type DbError = Error;

pub async fn init_db_pool(database_url: &str) -> DbPool {
    Pool::connect(database_url)
        .await
        .expect("Failed to initialize database pool")
}
