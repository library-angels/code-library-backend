use std::io;

#[macro_use]
extern crate diesel_migrations;

use book::{db, rpc_server};

use book::config::get_configuration;

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

    let database_url = configuration.get_db_connection_url();
    let db_pool = db::get_db_pool(&*database_url);

    embed_migrations!();
    helpers::db::run_migration(embedded_migrations::run, &db_pool);

    let (server, addr) = rpc_server(&configuration.get_service_socket(), db_pool)
        .await
        .unwrap();
    log::info!("Book RPC Server started on {}", addr);
    server.await;

    Ok(())
}
