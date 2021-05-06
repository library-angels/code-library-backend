use api::config::get_configuration;

#[tokio::main]
async fn main() {
    println!(
        "SERVICE: {} | VERSION: {}\n",
        option_env!("CARGO_PKG_NAME").unwrap_or("<unknown>"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("<unknown>")
    );
    env_logger::init();
    log::info!("Starting service");

    let configuration = get_configuration();

    let server = api::server(
        configuration.get_book_socket(),
        configuration.get_identity_socket(),
    )
    .bind(configuration.get_service_socket());
    log::info!(
        "API Server started on {}",
        configuration.get_service_socket()
    );
    server.await
}
