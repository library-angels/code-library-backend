use std::{io, process, sync::Arc};

use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use envconfig::Envconfig;
use futures::{future, prelude::*};
use tarpc::server::{self, Channel, Handler};
use tokio_serde::formats::Json;

#[macro_use]
extern crate diesel_migrations;

use identity::config::Configuration;
use identity::db::Db;
use identity::rpc::{server::IdentityServer, service::IdentityService};

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

    tarpc::serde_transport::tcp::listen(configuration.rpc_socket(), Json::default)
        .await?
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(11, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = IdentityServer {
                addr: channel.as_ref().as_ref().peer_addr().unwrap(),
                conf: configuration.clone(),
                db: db.clone(),
            };
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

    Ok(())
}
