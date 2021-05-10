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
use crate::db::DbPool;

type TcpTransport<T, U> =
    Transport<TcpStream, ClientMessage<T>, Response<U>, Json<ClientMessage<T>, Response<U>>>;

pub async fn rpc_client(addr: &SocketAddr) -> io::Result<BookServiceClient> {
    Ok(BookServiceClient::new(
        Config::default(),
        tarpc::serde_transport::tcp::connect(addr, Json::default).await?,
    )
    .spawn())
}

pub async fn rpc_server(
    addr: &SocketAddr,
    db_pool: DbPool,
) -> io::Result<(impl Future<Output = ()>, SocketAddr)> {
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
            let server = BookServer::new(db_pool.clone());
            channel.requests().execute(server.serve())
        })
        .buffer_unordered(MAX_CURRENT_CHANNEL)
        .for_each(|_| async {});

    Ok((server, addr))
}

#[cfg(test)]
mod tests {
    use std::net::TcpListener;

    use helpers::result::StdResult;

    use super::*;
    use crate::db;

    #[tokio::test]
    async fn rpc_server_binding_should_work() -> StdResult<()> {
        // Arrange
        let db_pool = db::get_db_pool(&format!(
            "postgres://postgres:password@{}",
            std::env::var("DB_SOCKET").unwrap()
        ));

        // Act
        let _ = rpc_server(&addr(), db_pool).await?;

        // Assert
        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn rpc_server_binding_should_fail() {
        // Arrange
        let socket = TcpListener::bind(addr()).unwrap();
        let addr = socket.local_addr().unwrap();

        let db_pool = db::get_db_pool(&format!(
            "postgres://postgres:password@{}",
            std::env::var("DB_SOCKET").unwrap()
        ));

        // Act
        let _ = rpc_server(&addr, db_pool).await.unwrap();

        // Assert
    }

    fn addr() -> SocketAddr {
        SocketAddr::from(([127, 0, 0, 1], 0))
    }
}
