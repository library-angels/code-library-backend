use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use futures::{future, prelude::*};
use once_cell::sync::OnceCell;
use std::io;
use std::net::SocketAddr;
use tarpc::server::{self, Channel, Handler};
use tokio_serde::formats::Json;

pub mod db;
pub mod rpc;

use crate::rpc::{server::BookServer, service::BookService};

pub static DB: OnceCell<Pool<ConnectionManager<PgConnection>>> = OnceCell::new();

pub async fn rpc_server(addr: &SocketAddr) -> io::Result<(impl Future<Output = ()>, SocketAddr)> {
    let incoming = tarpc::serde_transport::tcp::listen(addr, Json::default).await?;
    let addr = incoming.local_addr();

    let fut = incoming
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .max_channels_per_key(1, |t| t.as_ref().peer_addr().unwrap().ip())
        .map(|channel| {
            let server = BookServer(channel.as_ref().as_ref().peer_addr().unwrap());
            channel.respond_with(server.serve()).execute()
        })
        .buffer_unordered(10)
        .for_each(|_| async {});

    Ok((fut, addr))
}
