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
    oauth_client_identifier: String,
    oauth_client_secret: String,
    jwt_secret: String,
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
            db_socket: Configuration::init_db_socket()?,
            db_name: Configuration::init_db_name()?,
            db_username: Configuration::init_db_username()?,
            db_password: Configuration::init_db_password()?,
            oauth_client_identifier: Configuration::init_oauth_client_identifier()?,
            oauth_client_secret: Configuration::init_oauth_client_secret()?,
            jwt_secret: Configuration::init_jwt_secret()?,
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

    fn init_oauth_client_identifier() -> Result<String, ConfigurationError> {
        let key = "OAUTH_CLIENT_IDENTIFIER";
        match var(key) {
            Ok(password) => Ok(password),
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

    fn init_oauth_client_secret() -> Result<String, ConfigurationError> {
        let key = "OAUTH_CLIENT_SECRET";
        match var(key) {
            Ok(password) => Ok(password),
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

    fn init_jwt_secret() -> Result<String, ConfigurationError> {
        let key = "JWT_SECRET";
        match var(key) {
            Ok(password) => Ok(password),
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

    pub fn get_db_connection_base_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.db_username,
            self.db_password,
            self.db_socket.ip(),
            self.db_socket.port()
        )
    }

    pub fn get_oauth_client_identifier(&self) -> String {
        self.oauth_client_identifier.clone()
    }

    pub fn get_oauth_client_secret(&self) -> String {
        self.oauth_client_secret.clone()
    }

    pub fn get_jwt_secret(&self) -> String {
        self.jwt_secret.clone()
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5000),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "postgres".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "postgres".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
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
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_oauth_client_identifier_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_oauth_client_identifier_not_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");
        remove_var("OAUTH_CLIENT_IDENTIFIER");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let result = Configuration::init();

        assert_eq!(
            Err(ConfigurationError::new(
                "OAUTH_CLIENT_IDENTIFIER".into(),
                ErrorKind::EnvVarValueRequired
            )),
            result
        )
    }

    #[test]
    #[ignore]
    fn uts_oauth_client_secret_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_oauth_client_secret_not_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        remove_var("OAUTH_CLIENT_SECRET");
        set_var("JWT_SECRET", "jwt_secret");

        let result = Configuration::init();

        assert_eq!(
            Err(ConfigurationError::new(
                "OAUTH_CLIENT_SECRET".into(),
                ErrorKind::EnvVarValueRequired
            )),
            result
        )
    }

    #[test]
    #[ignore]
    fn uts_jwt_secret_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        set_var("JWT_SECRET", "jwt_secret");

        let expected_result = Configuration {
            service_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            db_socket: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432),
            db_name: "db_name".into(),
            db_username: "db_username".into(),
            db_password: "db_password".into(),
            oauth_client_identifier: "oauth_client_identifier".into(),
            oauth_client_secret: "oauth_client_secret".into(),
            jwt_secret: "jwt_secret".into(),
        };
        let result = Configuration::init();

        assert_eq!(Ok(expected_result), result)
    }

    #[test]
    #[ignore]
    fn uts_jwt_secret_not_set() {
        set_var("SERVICE_SOCKET", "127.0.0.1:8080");
        set_var("DB_SOCKET", "127.0.0.1:5432");
        set_var("DB_NAME", "db_name");
        set_var("DB_USERNAME", "db_username");
        set_var("DB_PASSWORD", "db_password");
        set_var("OAUTH_CLIENT_IDENTIFIER", "oauth_client_identifier");
        set_var("OAUTH_CLIENT_SECRET", "oauth_client_secret");
        remove_var("JWT_SECRET");

        let result = Configuration::init();

        assert_eq!(
            Err(ConfigurationError::new(
                "JWT_SECRET".into(),
                ErrorKind::EnvVarValueRequired
            )),
            result
        )
    }
}
