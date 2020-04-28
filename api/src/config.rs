use envconfig::{Envconfig, Error};
use envconfig_derive::Envconfig;
use log::error;
use std::{net::IpAddr, process, sync::Arc};

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "HTTP_HOST", default = "127.0.0.1")]
    pub http_host: IpAddr,

    #[envconfig(from = "HTTP_PORT", default = "8080")]
    pub http_port: u16,

    #[envconfig(
        from = "DATABASE_URL",
        defautl = "postgres://postgres:code@localhost:5432/code_library_development"
    )]
    pub database_url: String,
}

pub fn initialize_config() -> Result<Arc<Box<Config>>, Error> {
    match Config::init() {
        Ok(val) => Ok(Arc::new(Box::new(val))),
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    }
}
