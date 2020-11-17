use std::io;

use tarpc::client;
use tokio_serde::formats::Json;

use book_lib::service::BookServiceClient;
use identity_rpc_service::IdentityServiceClient;

use crate::CONFIGURATION;

pub async fn identity_client() -> io::Result<IdentityServiceClient> {
    let addr = CONFIGURATION.get().unwrap().identity_service_socket();
    IdentityServiceClient::new(
        client::Config::default(),
        tarpc::serde_transport::tcp::connect(addr, Json::default).await?,
    )
    .spawn()
}

pub async fn book_client() -> io::Result<BookServiceClient> {
    let addr = CONFIGURATION.get().unwrap().book_service_socket();
    BookServiceClient::new(
        client::Config::default(),
        tarpc::serde_transport::tcp::connect(addr, Json::default).await?,
    )
    .spawn()
}
