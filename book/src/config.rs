use envconfig::Envconfig;
use std::net::{IpAddr, SocketAddr};

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

    #[envconfig(from = "RPC_HOST_IP", default = "127.0.0.1")]
    rpc_host_ip: IpAddr,

    #[envconfig(from = "RPC_HOST_PORT", default = "8082")]
    rpc_host_port: u16,
}

impl Configuration {
    pub fn db_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}/{}",
            self.db_user, self.db_secret, self.db_host_ip, self.db_name
        )
    }

    pub fn rpc_socket(&self) -> SocketAddr {
        SocketAddr::new(self.rpc_host_ip, self.rpc_host_port)
    }
}
