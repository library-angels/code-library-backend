use super::service::*;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct NotificationServer(pub SocketAddr);

#[tarpc::server]
impl NotificationService for NotificationServer {
    async fn notification(self, _: context::Context) {
        unimplemented!();
    }
}
