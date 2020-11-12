use dotenv::dotenv;
use envconfig::Envconfig;
use helpers::log::log_error_and_panic;

use api_lib::{server, Configuration, CONFIGURATION};

static PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
static PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        PKG_NAME.unwrap_or("<unknown>"),
        PKG_VERSION.unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    dotenv().ok();
    let conf = Configuration::init_from_env()
        .map_err(log_error_and_panic)
        .unwrap();

    CONFIGURATION
        .set(conf)
        .map_err(|_| log_error_and_panic("Failed to provide global service configuration"))
        .unwrap();

    server()
        .try_bind(CONFIGURATION.get().unwrap().http_socket())
        .await;
}
