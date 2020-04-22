use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use std::net::IpAddr;


#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HTTP_HOST", default = "127.0.0.1")]
    pub http_host: IpAddr,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,

    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "OAUTH_CLIENT_IDENTIFIER")]
    pub oauth_client_identifier: String,

    #[envconfig(from = "OAUTH_CLIENT_SECRET")]
    pub oauth_client_secret: String,

    #[envconfig(from = "OAUTH_CLIENT_REDIRECT")]
    pub oauth_client_redirect: String,

    #[envconfig(from = "JWT_SECRET")]
    pub jwt_secret: String,
}