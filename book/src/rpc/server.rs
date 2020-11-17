use std::net::SocketAddr;
use tarpc::context;

use super::models::{Book, RpcResult};
use super::service::BookService;
use crate::db::queries;

#[derive(Clone)]
pub struct BookServer(pub SocketAddr);

#[tarpc::server]
impl BookService for BookServer {
    async fn get_book(self, _: context::Context, book_id: u32) -> RpcResult<Book> {
        Ok(queries::get_book_by_id(book_id as i32)?)
    }
}
