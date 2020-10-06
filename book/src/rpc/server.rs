use std::net::SocketAddr;
use tarpc::context;

use crate::db::models::{Book, Error};

use super::service::BookService;

#[derive(Clone)]
pub struct BookServer(pub SocketAddr);

#[tarpc::server]
impl BookService for BookServer {
    async fn get_book(self, _: context::Context, _book_id: u32) -> Result<Book, Error> {
        unimplemented!();
    }
}
