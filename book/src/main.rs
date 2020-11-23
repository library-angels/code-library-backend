mod config;

use std::{io, process};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use envconfig::Envconfig;

#[macro_use]
extern crate diesel_migrations;

use book::{rpc_server, DB};

use self::config::{Configuration, CONFIGURATION};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown>"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration = {
        dotenv().ok();
        match Configuration::init_from_env() {
            Ok(val) => val,
            Err(e) => {
                log::error!("{}", e);
                log::error!("Terminating because of previous error.");
                process::exit(1);
            }
        }
    };
    match CONFIGURATION.set(configuration) {
        Ok(()) => log::info!("Successfully provided global service configuration"),
        Err(_) => {
            log::error!("Failed to provide global service configuration");
            log::error!("Terminating because of previous error.");
            process::exit(1);
        }
    }

    let db: Pool<ConnectionManager<PgConnection>> = {
        match Pool::new(ConnectionManager::new(
            CONFIGURATION.get().unwrap().db_connection_url(),
        )) {
            Ok(val) => val,
            Err(e) => {
                log::error!("{}", e);
                log::error!("Terminating because of previous error.");
                process::exit(1);
            }
        }
    };
    match DB.set(db) {
        Ok(()) => log::info!("Successfully provided global database connection"),
        Err(_) => {
            log::error!("Failed to provide global database connection");
            log::error!("Terminating because of previous error.");
            process::exit(1);
        }
    }

    embed_migrations!();
    helpers::db::run_migration(embedded_migrations::run, &DB.get().unwrap());

    let (server, addr) = rpc_server(&CONFIGURATION.get().unwrap().rpc_socket())
        .await
        .unwrap();
    log::info!("Book RPC Server started on {}", addr);
    server.await;

    Ok(())
}
