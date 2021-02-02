use std::net::{IpAddr, SocketAddr};

use dotenv::dotenv;
use envconfig::Envconfig;

#[derive(Envconfig, Debug, Clone)]
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

    #[envconfig(from = "SERVICE_SOCKET", default = "127.0.0.1:8081")]
    service_socket: SocketAddr,

    #[envconfig(from = "OAUTH_CLIENT_IDENTIFIER")]
    pub oauth_client_identifier: String,

    #[envconfig(from = "OAUTH_CLIENT_SECRET")]
    pub oauth_client_secret: String,

    #[envconfig(from = "JWT_SECRET")]
    jwt_secret: String,
}

impl Configuration {
    pub fn db_connection_url(&self) -> String {
        format!("{}/{}", self.db_connection_base_url(), self.db_name)
    }

    pub fn db_connection_base_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}",
            self.db_user, self.db_secret, self.db_host_ip
        )
    }

    pub fn service_socket(&self) -> SocketAddr {
        self.service_socket
    }

    pub fn jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }
}

pub fn get_configuration() -> Configuration {
    dotenv().ok();
    Configuration::init_from_env().expect("Failed to create configuration")
}
