use crate::rpc::service::Book;
use std::net::SocketAddr;
use tarpc::context;

#[derive(Clone)]
pub struct BookService(pub SocketAddr);

#[tarpc::server]
impl Book for BookService {
    async fn book(self, _: context::Context) {
        unimplemented!();
    }
}
