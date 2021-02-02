use std::net::{IpAddr, SocketAddr};

use envconfig::Envconfig;

#[derive(Envconfig, Debug)]
pub struct Configuration {
    #[envconfig(from = "DB_HOST_IP", default = "127.0.0.1")]
    db_host_ip: IpAddr,

    #[envconfig(from = "DB_PORT", default = "5432")]
    db_port: u16,

    #[envconfig(from = "DB_NAME", default = "postgres")]
    db_name: String,

    #[envconfig(from = "DB_USER", default = "postgres")]
    db_user: String,

    #[envconfig(from = "DB_SECRET", default = "password")]
    db_secret: String,

    #[envconfig(from = "SERVICE_SOCKET", default = "127.0.0.1:8082")]
    service_socket: SocketAddr,
}

impl Configuration {
    pub fn db_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.db_user, self.db_secret, self.db_host_ip, self.db_name
        )
    }

    pub fn service_socket(&self) -> SocketAddr {
        self.service_socket
    }
}
