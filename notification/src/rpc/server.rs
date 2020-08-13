use crate::rpc::service::Notification;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct NotificationService(pub SocketAddr);

#[tarpc::server]
impl Notification for NotificationService {
    async fn notification(self, _: context::Context) {
        unimplemented!();
    }
}
