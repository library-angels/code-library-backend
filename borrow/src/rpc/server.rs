use crate::rpc::service::Borrow;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct BorrowService(pub SocketAddr);

#[tarpc::server]
impl Borrow for BorrowService {
    async fn borrow(self, _: context::Context) {
        unimplemented!();
    }
}
