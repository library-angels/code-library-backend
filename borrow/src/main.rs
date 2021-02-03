use std::{io, sync::Arc};

#[macro_use]
extern crate diesel_migrations;

use borrow::{config::get_configuration, db::get_db_pool, rpc::get_rpc_server};
use helpers::db::run_migration;

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

    let db_pool = get_db_pool(&configuration.db_connection_url());

    embed_migrations!();
    run_migration(embedded_migrations::run, &db_pool);

    let (server, addr) = get_rpc_server(
        configuration.service_socket(),
        Arc::new(configuration),
        Arc::new(db_pool),
    )
    .await
    .unwrap();
    log::info!("Borrow RPC Server started on {}", addr);
    server.await;

    Ok(())
}
