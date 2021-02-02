use dotenv::dotenv;
use envconfig::Envconfig;

use api::config::Configuration;

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
    let conf = Configuration::init_from_env().unwrap();
    let addr = conf.api_socket;
    let book_addr = conf.book_socket;
    let identity_addr = conf.identity_socket;

    let server = api::server(book_addr, identity_addr).bind(addr);
    log::info!("API Server started on {}", addr);
    server.await
}
