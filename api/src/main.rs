use config::Configuration;
use once_cell::sync::OnceCell;
use std::process;
use warp::Filter;
extern crate log;
use dotenv::dotenv;
use envconfig::Envconfig;

mod config;
mod endpoints;
mod router;
mod rpc;

static PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
static PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();

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

    let routes = router::router().with(
        warp::cors()
            .allow_any_origin()
            .allow_methods(vec!["GET", "POST", "DELETE"])
            .allow_headers(vec!["Content-Type"]),
    );

    warp::serve(routes)
        .try_bind(CONFIGURATION.get().unwrap().http_socket())
        .await;
}
