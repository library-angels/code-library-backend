use diesel::result::Error as DBError;
use std::net::SocketAddr;
use tarpc::context;

use super::models::{Book, Error};
use super::service::BookService;
use crate::db::queries;

pub use helpers::rpc::RpcResult;

#[derive(Clone)]
pub struct BookServer(pub SocketAddr);

#[tarpc::server]
impl BookService for BookServer {
    async fn get_book(self, _: context::Context, book_id: u32) -> RpcResult<Book> {
        match queries::get_book_by_id(book_id as i32) {
            Ok(book) => Ok(book),
            Err(e) => {
                log::debug!("Error (BookServer::get_book): {}", e);
                match e {
                    DBError::NotFound => Err(Error::NotFound),
                    _ => Err(Error::InternalError),
                }
            }
        }
    }
}
