use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel_migrations::RunMigrationsError;

pub fn run_migration(
    run_embedded_migration: impl FnOnce(
        &PooledConnection<ConnectionManager<PgConnection>>,
    ) -> Result<(), RunMigrationsError>,
    db: &Pool<ConnectionManager<PgConnection>>,
) {
    match run_embedded_migration(&db.get().unwrap()) {
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
