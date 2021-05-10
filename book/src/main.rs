use std::io;

use book::rpc_server;

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

    let (server, addr) = rpc_server(&configuration.get_service_socket())
        .await
        .unwrap();
    log::info!("Book RPC Server started on {}", addr);
    server.await;

    Ok(())
}
