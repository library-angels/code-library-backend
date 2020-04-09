use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use std::net::IpAddr;


#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HTTP_HOST", default = "127.0.0.1")]
    pub http_host: IpAddr,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,

    #[envconfig(from = "OAUTH_CLIENT_IDENTIFIER")]
    pub oauth_client_identifier: String,
}