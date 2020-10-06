#![allow(clippy::redundant_closure)]

use std::io;

use tarpc::client;
use tokio_serde::formats::Json;

use book_lib::service::BookServiceClient;

use crate::CONFIGURATION;

pub async fn identity_client() -> Result<identity_rpc_service::IdentityServiceClient, std::io::Error>
{
    let transport = tarpc::serde_transport::tcp::connect(
        CONFIGURATION.get().unwrap().identity_service_socket(),
        || Json::default(),
    )
    .await;
    match transport {
        Ok(val) => {
            identity_rpc_service::IdentityServiceClient::new(client::Config::default(), val).spawn()
        }
        Err(e) => Err(e),
    }
}

pub async fn book_client() -> io::Result<BookServiceClient> {
    let addr = CONFIGURATION.get().unwrap().book_service_socket();
    BookServiceClient::new(
        client::Config::default(),
        tarpc::serde_transport::tcp::connect(addr, || Json::default()).await?,
    )
    .spawn()
}
