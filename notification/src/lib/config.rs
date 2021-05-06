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
    db_socket: SocketAddr,
    db_name: String,
    db_username: String,
    db_password: String,
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
    SocketAddrInvalid,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::EnvVarNotSet => write!(f, "EnvVarNotSet"),
            ErrorKind::EnvVarValueInvalid => write!(f, "EnvVarValueInvalid"),
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
            db_socket: Configuration::init_db_socket()?,
            db_name: Configuration::init_db_name()?,
            db_username: Configuration::init_db_username()?,
            db_password: Configuration::init_db_password()?,
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

    fn init_db_socket() -> Result<SocketAddr, ConfigurationError> {
        let key = "DB_SOCKET";
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
            Err(VarError::NotPresent) => {
                log::debug!("Environment variable {} uses default value.", key);
                Ok(SocketAddr::new(
                    IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    5432,
                ))
            }
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    fn init_db_name() -> Result<String, ConfigurationError> {
        let key = "DB_NAME";
        match var(key) {
            Ok(name) => Ok(name),
            Err(VarError::NotPresent) => Ok(String::from("postgres")),
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    fn init_db_username() -> Result<String, ConfigurationError> {
        let key = "DB_USERNAME";
        match var(key) {
            Ok(username) => Ok(username),
            Err(VarError::NotPresent) => Ok(String::from("postgres")),
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    fn init_db_password() -> Result<String, ConfigurationError> {
        let key = "DB_PASSWORD";
        match var(key) {
            Ok(password) => Ok(password),
            Err(VarError::NotPresent) => Ok(String::from("password")),
            Err(VarError::NotUnicode(_)) => Err(ConfigurationError::new(
                key.into(),
                ErrorKind::EnvVarValueInvalid,
            )),
        }
    }

    pub fn get_service_socket(&self) -> SocketAddr {
        self.service_socket
    }

    pub fn get_db_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_username,
            self.db_password,
            self.db_socket.ip(),
            self.db_socket.port(),
            self.db_name
        )
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
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_service_socket_input_valid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8000");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_service_socket_input_invalid() {
        set_var("SERVICE_SOCKET", "127.0.0.1");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result =
            ConfigurationError::new("SERVICE_SOCKET".into(), ErrorKind::SocketAddrInvalid);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_socket_default() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        remove_var("DB_SOCKET");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_socket_input_valid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5000");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_socket_input_invalid() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result =
            ConfigurationError::new("DB_SOCKET".into(), ErrorKind::SocketAddrInvalid);
        let result = Configuration::init();

        assert_eq!(Err(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_name_default() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        remove_var("DB_NAME");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "postgres".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_name_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_username_default() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        remove_var("DB_USERNAME");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "postgres".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_username_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_password_default() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        remove_var("DB_PASSWORD");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_db_password_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }
}
