use std::{io, net::SocketAddr};

use tarpc::client;
use tokio_serde::formats::Json;

use book::service::BookServiceClient;
use identity::rpc::service::IdentityServiceClient;

pub async fn identity_client(addr: &SocketAddr) -> io::Result<IdentityServiceClient> {
    IdentityServiceClient::new(
        client::Config::default(),
        tarpc::serde_transport::tcp::connect(addr, Json::default).await?,
    )
    .spawn()
}

pub async fn book_client(addr: &SocketAddr) -> io::Result<BookServiceClient> {
    book::rpc_client(addr).await
}
