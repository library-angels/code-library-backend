use dotenv::dotenv;
use envconfig::Envconfig;

use api::{server, Configuration, CONFIGURATION};

#[tokio::main]
async fn main() {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown>"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    dotenv().ok();
    let configuration = Configuration::init_from_env().unwrap();
    CONFIGURATION
        .set(configuration)
        .expect("Failed to provide global service configuration");

    let addr = CONFIGURATION.get().unwrap().http_socket();
    let server = server().bind(addr);
    log::info!("API Server started on {}", addr);
    server.await
}
