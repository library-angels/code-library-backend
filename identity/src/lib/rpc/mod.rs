pub mod models;
pub mod server;
pub mod service;

use std::{io, net::SocketAddr, sync::Arc};

use futures::{future, prelude::*};
use tarpc::client::Config;
use tarpc::server::{BaseChannel, Channel, Handler};
use tokio_serde::formats::Json;

use self::server::IdentityServer;
use self::service::{IdentityService, IdentityServiceClient};
use crate::{config::Configuration, db::DbPool};

pub async fn rpc_server(
    addr: SocketAddr,
    configuration: Arc<Configuration>,
    db_pool: Arc<DbPool>,
) -> io::Result<(impl Future<Output = ()>, SocketAddr)> {
    let incoming = tarpc::serde_transport::tcp::listen(&addr, Json::default).await?;
    let addr = incoming.local_addr();

    let fut = incoming
        .filter_map(|r| future::ready(r.ok()))
        .map(BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(move |channel| {
            let server = IdentityServer {
                addr: channel.as_ref().as_ref().peer_addr().unwrap(),
                conf: configuration.clone(),
                db_pool: db_pool.clone(),
            };
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {});

    Ok((fut, addr))
}

pub async fn rpc_client(addr: SocketAddr) -> std::io::Result<IdentityServiceClient> {
    IdentityServiceClient::new(
        Config::default(),
        tarpc::serde_transport::tcp::connect(addr, Json::default).await?,
    )
    .spawn()
}
