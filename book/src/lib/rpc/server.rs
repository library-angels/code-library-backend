use tarpc::context::Context;
use uuid::Uuid;

use helpers::rpc::RpcResult;

use super::models::Book;
use super::service::BookService;

#[derive(Clone)]
pub struct BookServer {}

impl BookServer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for BookServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tarpc::server]
impl BookService for BookServer {
    async fn get_book(self, _: Context, _id: Uuid) -> RpcResult<Book> {
        unimplemented!();
    }

    async fn list_books(self, _: Context) -> RpcResult<Vec<Book>> {
        unimplemented!();
    }
}
