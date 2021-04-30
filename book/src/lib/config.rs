use std::{
    env,
    ffi::OsString,
    fmt,
    net::{SocketAddr, ToSocketAddrs},
};

use dotenv::dotenv;

const SERVICE_SOCKET_DEFAULT: &str = "127.0.0.1:8082";
const DB_HOST_SOCKET_DEFAULT: &str = "127.0.0.1:5432";
const DB_NAME: &str = "postgres";
const DB_USER: &str = "postgres";
const DB_SECRET: &str = "password";

pub enum Error<'a> {
    InvalidData(&'a str),
    ResolveSocketAddress(&'a str),
}

impl<'a> fmt::Debug for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidData(e) => {
                write!(f, "Environment variable {} contained invalid content", e)
            }
            Error::ResolveSocketAddress(e) => {
                write!(f, "Socket for variable {} could not be resolved", e)
            }
        }
    }
}

pub struct Configuration {
    service_socket: SocketAddr,
    db_host_socket: SocketAddr,
    db_name: String,
    db_user: String,
    db_secret: String,
}

impl<'a> Configuration {
    pub fn init() -> Result<Self, Error<'a>> {
        let service_socket = Configuration::parse_service_socket(env::var_os("SERVICE_SOCKET"))?;
        let db_host_socket = Configuration::parse_db_host_socket(env::var_os("DB_HOST_SOCKET"))?;
        let db_name = Configuration::parse_db_name(env::var_os("DB_NAME"))?;
        let db_user = Configuration::parse_db_user(env::var_os("DB_USER"))?;
        let db_secret = Configuration::parse_db_secret(env::var_os("DB_SECRET"))?;

        Ok(Configuration {
            service_socket,
            db_host_socket,
            db_name,
            db_user,
            db_secret,
        })
    }

    fn parse_service_socket(env_var: Option<OsString>) -> Result<SocketAddr, Error<'a>> {
        if let Some(var) = env_var {
            let value = var
                .into_string()
                .map_err(|_| Error::InvalidData("SERVICE_SOCKET"))?;
            Ok(value
                .to_socket_addrs()
                .map_err(|_| Error::ResolveSocketAddress("SERVICE_SOCKET"))?
                .filter(|addr| addr.is_ipv4())
                .collect::<Vec<SocketAddr>>()
                .pop()
                .unwrap())
        } else {
            Ok(SERVICE_SOCKET_DEFAULT.parse().unwrap())
        }
    }

    fn parse_db_host_socket(env_var: Option<OsString>) -> Result<SocketAddr, Error<'a>> {
        if let Some(var) = env_var {
            let value = var
                .into_string()
                .map_err(|_| Error::InvalidData("DB_HOST_SOCKET"))?;
            Ok(value
                .to_socket_addrs()
                .map_err(|_| Error::ResolveSocketAddress("DB_HOST_SOCKET"))?
                .filter(|addr| addr.is_ipv4())
                .collect::<Vec<SocketAddr>>()
                .pop()
                .unwrap())
        } else {
            Ok(DB_HOST_SOCKET_DEFAULT.parse().unwrap())
        }
    }

    fn parse_db_name(env_var: Option<OsString>) -> Result<String, Error<'a>> {
        if let Some(var) = env_var {
            var.into_string().map_err(|_| Error::InvalidData("DB_NAME"))
        } else {
            Ok(DB_NAME.parse().unwrap())
        }
    }

    fn parse_db_user(env_var: Option<OsString>) -> Result<String, Error<'a>> {
        if let Some(var) = env_var {
            var.into_string().map_err(|_| Error::InvalidData("DB_USER"))
        } else {
            Ok(DB_USER.parse().unwrap())
        }
    }

    fn parse_db_secret(env_var: Option<OsString>) -> Result<String, Error<'a>> {
        if let Some(var) = env_var {
            var.into_string()
                .map_err(|_| Error::InvalidData("DB_SECRET"))
        } else {
            Ok(DB_SECRET.parse().unwrap())
        }
    }

    pub fn get_db_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user,
            self.db_secret,
            self.db_host_socket.ip(),
            self.db_host_socket.port(),
            self.db_name
        )
    }

    pub fn get_service_socket(&self) -> SocketAddr {
        self.service_socket
    }
}

pub fn get_configuration() -> Configuration {
    dotenv().ok();
    Configuration::init().expect("Failed to create configuration")
}
