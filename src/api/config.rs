use dotenv::dotenv;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use log::{debug, info};
use serial_test::serial;


const HTTP_HOST_DEFAULT: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const HTTP_PORT_DEFAULT: u16 = 8080;

#[derive(Debug, PartialEq)]
pub struct Config {
    http_host: Option<IpAddr>,
    http_port: Option<u16>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    HttpHostInvalidValue,
    HttpPortInvalidValue,
}

impl Config {
    pub fn new() -> Result<Config, Error> {
        dotenv().ok();

        let mut config = Config {
            http_host: Some(HTTP_HOST_DEFAULT),
            http_port: Some(HTTP_PORT_DEFAULT),
        };

        // HTTP_HOST
        match env::var("HTTP_HOST") {
            Ok(val) => {
                match val.parse::<Ipv4Addr>() {
                    Ok(val) => config.http_host = Some(IpAddr::V4(val)),
                    Err(_) => return Err(Error::HttpHostInvalidValue)
                }
            }
            Err(_) => debug!("'HTTP_HOST' not set or not valid unicode, using default value")
        }

        // HTTP_PORT
        match env::var("HTTP_PORT") {
            Ok(val) => {
                match val.parse::<u16>() {
                    Ok(val) => config.http_port = Some(val),
                    Err(_) => return Err(Error::HttpPortInvalidValue)
                }
            }
            Err(_) => debug!("'HTTP_PORT' not set or not valid unicode, using default value")
        }

        return Ok(config);
    }

    pub fn http_socket(&self) -> SocketAddr {
        SocketAddr::new(self.http_host.unwrap(), self.http_port.unwrap())
    }

    pub fn log_info_configuration(&self) {
        info!("'HTTP_HOST' - {}", self.http_host.unwrap());
        info!("'HTTP_PORT' - {}", self.http_port.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn no_env_defaults() {
        env::remove_var("HTTP_HOST");
        env::remove_var("HTTP_PORT");
        let config = Config::new().unwrap();
        assert_eq!(config.http_socket(), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080));
    }

    #[test]
    #[serial]
    fn invalid_http_host() {
        env::remove_var("HTTP_PORT");
        env::set_var("HTTP_HOST", "abc");
        assert_eq!(Config::new(), Err(Error::HttpHostInvalidValue));
    }

    #[test]
    #[serial]
    fn invalid_http_port() {
        env::remove_var("HTTP_HOST");
        env::set_var("HTTP_PORT", "abc");
        assert_eq!(Config::new(), Err(Error::HttpPortInvalidValue));
    }

    #[test]
    #[serial]
    fn set_http_host() {
        env::remove_var("HTTP_PORT");
        env::set_var("HTTP_HOST", "127.0.0.2");
        let config = Config::new().unwrap();
        assert_eq!(config.http_socket(), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8080));
    }

    #[test]
    #[serial]
    fn set_http_port() {
        env::remove_var("HTTP_HOST");
        env::set_var("HTTP_PORT", "8081");
        let config = Config::new().unwrap();
        assert_eq!(config.http_socket(), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081));
    }

    #[test]
    #[serial]
    fn set_http_host_port() {
        env::set_var("HTTP_HOST", "127.0.0.2");
        env::set_var("HTTP_PORT", "8081");
        let config = Config::new().unwrap();
        assert_eq!(config.http_socket(), SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 8081));
    }
}