extern crate log;
use dotenv::dotenv;
use envconfig::Envconfig;
use once_cell::sync::OnceCell;
use std::process;
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use config::Configuration;

mod config;

static PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
static PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();
static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();

#[tokio::main]
async fn main() {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        PKG_NAME.unwrap_or("<unknown>"),
        PKG_VERSION.unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration = {
        dotenv().ok();
        match Configuration::init() {
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
}
