use config::Configuration;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use envconfig::Envconfig;
use futures::{future, prelude::*};
use once_cell::sync::OnceCell;
use rpc::server::BookServer;
use rpc::service::BookService;
use std::io;
use std::process;
use tarpc::server::{self, Channel, Handler};
use tokio_serde::formats::Json;

#[macro_use]
extern crate diesel_migrations;

mod config;
mod db;
mod rpc;

static PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
static PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();
static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();

#[tokio::main]
async fn main() -> io::Result<()> {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        PKG_NAME.unwrap_or("<unknown>"),
        PKG_VERSION.unwrap_or("<unknown>")
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

    tarpc::serde_transport::tcp::listen(&CONFIGURATION.get().unwrap().rpc_socket(), Json::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = BookServer(channel.as_ref().as_ref().peer_addr().unwrap());
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
