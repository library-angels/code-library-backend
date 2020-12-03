use std::{io, sync::Arc};

#[macro_use]
extern crate diesel_migrations;

use identity::{config::configuration, db::db_pool, rpc::rpc_server};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown>"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration = configuration();

    let db_pool = db_pool(&configuration.db_connection_url());

    embed_migrations!();
    helpers::db::run_migration(embedded_migrations::run, &db_pool);

    let (server, addr) = rpc_server(
        configuration.rpc_socket(),
        Arc::new(configuration),
        Arc::new(db_pool),
    )
    .await
    .unwrap();
    log::info!("Identity RPC Server started on {}", addr);
    server.await;

    Ok(())
}
