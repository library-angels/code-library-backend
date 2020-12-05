use envconfig::Envconfig;
use std::net::SocketAddr;

#[derive(Debug, Envconfig)]
pub struct Configuration {
    #[envconfig(from = "API_SOCKET", default = "127.0.0.1:8080")]
    pub api_socket: SocketAddr,

    #[envconfig(from = "IDENTITY_SOCKET", default = "127.0.0.1:8081")]
    pub identity_socket: SocketAddr,

    #[envconfig(from = "BOOK_SOCKET", default = "127.0.0.1:8082")]
    pub book_socket: SocketAddr,
}
