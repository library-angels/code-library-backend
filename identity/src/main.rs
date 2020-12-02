use std::{io, sync::Arc};

#[macro_use]
extern crate diesel_migrations;

use identity::config::{configuration, Configuration};
use identity::db::{db_pool, DbPool};
use identity::rpc::rpc_server;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown>"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration: Arc<Configuration> = Arc::new(configuration());

    let db_pool: Arc<DbPool> = Arc::new(db_pool(&configuration.db_connection_url()));

    embed_migrations!();
    helpers::db::run_migration(embedded_migrations::run, &db_pool);

    let (server, addr) = rpc_server(
        configuration.rpc_socket(),
        configuration.clone(),
        db_pool.clone(),
    )
    .await
    .unwrap();
    log::info!("Identity RPC Server started on {}", addr);
    server.await;

    Ok(())
}
