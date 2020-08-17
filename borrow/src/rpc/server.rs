use super::service::*;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct BorrowServer(pub SocketAddr);

#[tarpc::server]
impl BorrowService for BorrowServer {
    async fn borrow(self, _: context::Context) {
        unimplemented!();
    }
}
