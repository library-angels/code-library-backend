use std::{io, process, sync::Arc};

use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use envconfig::Envconfig;

#[macro_use]
extern crate diesel_migrations;

use identity::config::Configuration;
use identity::db::Db;
use identity::rpc::rpc_server;

static PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
static PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        PKG_NAME.unwrap_or("<unknown>"),
        PKG_VERSION.unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration: Arc<Configuration> = {
        dotenv().ok();
        match Configuration::init_from_env() {
            Ok(val) => Arc::new(val),
            Err(e) => {
                log::error!("{}", e);
                log::error!("Terminating because of previous error.");
                process::exit(1);
            }
        }
    };

    let db: Arc<Db> = {
        match Pool::new(ConnectionManager::new(configuration.db_connection_url())) {
            Ok(val) => Arc::new(val),
            Err(e) => {
                log::error!("{}", e);
                log::error!("Terminating because of previous error.");
                process::exit(1);
            }
        }
    };

    embed_migrations!();
    helpers::db::run_migration(embedded_migrations::run, &db);

    let (server, addr) = rpc_server(
        configuration.rpc_socket(),
        configuration.clone(),
        db.clone(),
    )
    .await
    .unwrap();
    log::info!("Identity RPC Server started on {}", addr);
    server.await;

    Ok(())
}
