pub mod models;
pub mod server;
pub mod service;

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use futures::{future, prelude::*};
use tarpc::server::{BaseChannel, Channel, Handler};
use tokio_serde::formats::Json;

use self::{server::BookServer, service::BookService};
use crate::db::DbPool;

pub async fn rpc_server(
    addr: &SocketAddr,
    db_pool: Arc<DbPool>,
) -> io::Result<(impl Future<Output = ()>, SocketAddr)> {
    let incoming = tarpc::serde_transport::tcp::listen(addr, Json::default).await?;
    let addr = incoming.local_addr();

    let server = incoming
        .filter_map(|r| future::ready(r.ok()))
        .map(BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(move |channel| {
            let server = BookServer::new(
                channel.as_ref().as_ref().peer_addr().unwrap(),
                Arc::clone(&db_pool),
            );
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {});

    Ok((server, addr))
}
