use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use std::net::{IpAddr, SocketAddr};

#[derive(Envconfig, Debug)]
pub struct Configuration {
    #[envconfig(from = "HTTP_HOST_IP", default = "127.0.0.1")]
    http_host_ip: IpAddr,

    #[envconfig(from = "HTTP_HOST_PORT", default = "8080")]
    http_host_port: u16,

    #[envconfig(from = "IDENTITY_SERVICE_HOST_IP", default = "127.0.0.1")]
    identity_service_host_ip: IpAddr,

    #[envconfig(from = "IDENTITY_SERVICE_HOST_PORT", default = "8081")]
    identity_service_host_port: u16,
}

impl Configuration {
    pub fn http_socket(&self) -> SocketAddr {
        SocketAddr::new(self.http_host_ip, self.http_host_port)
    }

    pub fn identity_service_socket(&self) -> SocketAddr {
        SocketAddr::new(self.identity_service_host_ip, self.identity_service_host_port)
    }
}
