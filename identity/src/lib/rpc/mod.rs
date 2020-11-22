pub mod server;
pub mod service;

use std::{io, net::SocketAddr, sync::Arc};

use futures::{future, prelude::*};
use tarpc::server::{BaseChannel, Channel, Handler};
use tokio_serde::formats::Json;

use self::server::IdentityServer;
use self::service::IdentityService;
use crate::{config::Configuration, db::Db};

pub async fn rpc_server(
    addr: SocketAddr,
    configuration: Arc<Configuration>,
    db: Arc<Db>,
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
                db: db.clone(),
            };
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {});

    Ok((fut, addr))
}
