use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use std::net::IpAddr;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "DB_HOST_IP")]
    db_host_ip: IpAddr,

    #[envconfig(from = "DB_PORT", default = "5432")]
    db_port: u16,

    #[envconfig(from = "DB_NAME")]
    db_name: String,

    #[envconfig(from = "DB_USER")]
    db_user: String,

    #[envconfig(from = "DB_SECRET")]
    db_secret: String,
}

impl Config {
    pub fn db_connection_url(&self) -> String {
        format!("postgres://{}:{}@{}/{}",
            self.db_user,
            self.db_secret,
            self.db_host_ip,
            self.db_name
        ).to_string()
    }
}
