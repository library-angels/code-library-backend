use dotenv::dotenv;

use std::{
    env::{var, VarError},
    fmt,
    net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub struct Configuration {
    service_socket: SocketAddr,
    identity_socket: SocketAddr,
    book_socket: SocketAddr,
}

#[derive(Debug, PartialEq)]
pub struct ConfigurationError {
    description: String,
    kind: ErrorKind,
}

impl ConfigurationError {
    pub fn new(description: String, kind: ErrorKind) -> Self {
        ConfigurationError { description, kind }
    }
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.kind, self.description)
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    EnvVarNotSet,
    EnvVarValueInvalid,
    EnvVarValueRequired,
    SocketAddrInvalid,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::EnvVarNotSet => write!(f, "EnvVarNotSet"),
            ErrorKind::EnvVarValueInvalid => write!(f, "EnvVarValueInvalid"),
            ErrorKind::EnvVarValueRequired => write!(f, "EnvVarValueRequired"),
            ErrorKind::SocketAddrInvalid => write!(f, "SocketAddrInvalid"),
        }
    }
}

impl From<VarError> for ErrorKind {
    fn from(error: VarError) -> Self {
        match error {
            VarError::NotPresent => ErrorKind::EnvVarNotSet,
            VarError::NotUnicode(_) => ErrorKind::EnvVarValueInvalid,
        }
    }
}

impl Configuration {
    pub fn init() -> Result<Self, ConfigurationError> {
        Ok(Self {
            service_socket: Configuration::init_service_socket()?,
            book_socket: Configuration::init_book_socket()?,
            identity_socket: Configuration::init_identity_socket()?,
        })
    }

    fn init_service_socket() -> Result<SocketAddr, ConfigurationError> {
        let key = "SERVICE_SOCKET";
        match var(key) {
            Ok(socket) => Ok(SocketAddr::from_str(&socket)
                .map_err(|_| ConfigurationError::new(key.into(), ErrorKind::SocketAddrInvalid))?),
            Err(VarError::NotPresent) => {
                log::debug!("Environment variable {} uses default value.", key);
                Ok(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    8080,
                ))
            }
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    fn init_identity_socket() -> Result<SocketAddr, ConfigurationError> {
        let key = "IDENTITY_SOCKET";
        match var(key) {
            Ok(socket) => {
                let sockets = socket
                    .to_socket_addrs()
                    .map_err(|_| ConfigurationError::new(key.into(), ErrorKind::SocketAddrInvalid))?
                    .filter(|socket| socket.is_ipv4())
                    .collect::<Vec<SocketAddr>>();
                Ok(*sockets.get(0).ok_or_else(|| {
                    ConfigurationError::new(key.into(), ErrorKind::SocketAddrInvalid)
                })?)
            }
            Err(VarError::NotPresent) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueRequired,
            )),
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    fn init_book_socket() -> Result<SocketAddr, ConfigurationError> {
        let key = "BOOK_SOCKET";
        match var(key) {
            Ok(socket) => {
                let sockets = socket
                    .to_socket_addrs()
                    .map_err(|_| ConfigurationError::new(key.into(), ErrorKind::SocketAddrInvalid))?
                    .filter(|socket| socket.is_ipv4())
                    .collect::<Vec<SocketAddr>>();
                Ok(*sockets.get(0).ok_or_else(|| {
                    ConfigurationError::new(key.into(), ErrorKind::SocketAddrInvalid)
                })?)
            }
            Err(VarError::NotPresent) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueRequired,
            )),
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    pub fn get_service_socket(&self) -> SocketAddr {
        self.service_socket
    }

    pub fn get_identity_socket(&self) -> SocketAddr {
        self.identity_socket
    }

    pub fn get_book_socket(&self) -> SocketAddr {
        self.book_socket
    }
}

pub fn get_configuration() -> Configuration {
    dotenv().ok();
    Configuration::init().expect("Failed to create configuration")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{remove_var, set_var};

    #[test]
    #[ignore]
    fn uts_service_socket_default() {
        remove_var("SERVICE_SOCKET");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            identity_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
            book_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_service_socket_input_valid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8000");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
            identity_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
            book_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_service_socket_input_invalid() {
        set_var("SERVICE_SOCKET", "127.0.0.1");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result =
            ConfigurationError::new("SERVICE_SOCKET".into(), ErrorKind::SocketAddrInvalid);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_identity_socket_input_valid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            identity_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
            book_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_identity_socket_input_invalid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8082");
        set_var("IDENTITY_SOCKET", "127.0.0.1");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result =
            ConfigurationError::new("IDENTITY_SOCKET".into(), ErrorKind::SocketAddrInvalid);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_identity_socket_input_not_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        remove_var("IDENTITY_SOCKET");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result =
            ConfigurationError::new("IDENTITY_SOCKET".into(), ErrorKind::EnvVarValueRequired);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_book_socket_input_valid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        set_var("BOOK_SOCKET", "127.0.0.1:8082");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            identity_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8081),
            book_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8082),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_book_socket_input_invalid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8082");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        set_var("BOOK_SOCKET", "127.0.0.1");

        let expected_result =
            ConfigurationError::new("BOOK_SOCKET".into(), ErrorKind::SocketAddrInvalid);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_book_socket_input_not_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("IDENTITY_SOCKET", "127.0.0.1:8081");
        remove_var("BOOK_SOCKET");

        let expected_result =
            ConfigurationError::new("BOOK_SOCKET".into(), ErrorKind::EnvVarValueRequired);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }
}
