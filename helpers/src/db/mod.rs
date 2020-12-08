pub mod pagination;

use diesel::{
    pg::{Pg, PgConnection},
    r2d2::{ConnectionManager, Pool, PooledConnection},
    Connection,
};
use diesel_migrations::RunMigrationsError;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_db_pool(database_url: &str) -> DbPool {
    Pool::new(ConnectionManager::new(database_url))
        .expect("Failed creating new database connection")
}

pub fn run_migration<T, U>(run_embedded_migration: T, conn: &U)
where
    T: FnOnce(&U) -> Result<(), RunMigrationsError>,
    U: Connection<Backend = Pg>,
{
    match run_embedded_migration(conn) {
        Ok(()) => log::info!("Successfully applied migrations on database."),
        Err(err) => {
            let error_type = match &err {
                RunMigrationsError::MigrationError(_) => "Migration Error",
                RunMigrationsError::QueryError(_) => "Query Error",
                RunMigrationsError::EmptyMigration => "Empty Migration",
                RunMigrationsError::__NonExhaustive => "Unknown Error",
            };
            log::error!(
                "Failed to apply migration on database ({}): {}",
                error_type,
                err
            );
            log::error!("Terminating because of previous error.");
            panic!();
        }
    }
}
