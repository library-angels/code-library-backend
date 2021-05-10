pub mod models;
pub mod server;
pub mod service;

use std::io;
use std::net::SocketAddr;

use futures::{future, prelude::*};
use tarpc::{
    client::Config,
    serde_transport::{tcp, Transport},
    server::{BaseChannel, Channel, Incoming},
    ClientMessage, Response,
};
use tokio::net::TcpStream;
use tokio_serde::formats::Json;

use self::server::BookServer;
use self::service::{BookService, BookServiceClient};

type TcpTransport<T, U> =
    Transport<TcpStream, ClientMessage<T>, Response<U>, Json<ClientMessage<T>, Response<U>>>;

pub async fn rpc_client(addr: &SocketAddr) -> io::Result<BookServiceClient> {
    Ok(BookServiceClient::new(
        Config::default(),
        tarpc::serde_transport::tcp::connect(addr, Json::default).await?,
    )
    .spawn())
}

pub async fn rpc_server(addr: &SocketAddr) -> io::Result<(impl Future<Output = ()>, SocketAddr)> {
    const CHANNEL_PER_IP: u32 = 10;
    const MAX_CURRENT_CHANNEL: usize = 10;

    let incoming = tcp::listen(addr, Json::default).await?;
    let addr = incoming.local_addr();

    let keymaker = |t: &BaseChannel<_, _, TcpTransport<_, _>>| t.as_ref().peer_addr().unwrap().ip();

    let server = incoming
        .filter_map(|r: io::Result<TcpTransport<_, _>>| future::ready(r.ok()))
        .map(BaseChannel::with_defaults)
        .max_channels_per_key(CHANNEL_PER_IP, keymaker)
        .map(move |channel| {
            let server = BookServer::default();
            channel.requests().execute(server.serve())
        })
        .buffer_unordered(MAX_CURRENT_CHANNEL)
        .for_each(|_| async {});

    Ok((server, addr))
}
