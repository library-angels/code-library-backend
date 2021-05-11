use std::io;

use book::{config::get_configuration, db::init_db_pool, rpc_server};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown>"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration = get_configuration();
    let db_pool = init_db_pool(&configuration.get_db_connection_url()).await;

    let mut migrator = sqlx::migrate!();
    migrator
        .migrations
        .to_mut()
        .retain(|migration| !migration.migration_type.is_down_migration());
    let migration_result = migrator.run(&db_pool).await;
    match migration_result {
        Ok(()) => {
            log::info!("Database migration applied successfully")
        }
        Err(e) => {
            log::error!("Database migration failed with error: {}", e);
            panic!();
        }
    }

    let (server, addr) = rpc_server(&configuration.get_service_socket()).await?;
    log::info!("Book RPC Server started on {}", addr);
    server.await;

    Ok(())
}
