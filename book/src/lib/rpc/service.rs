use uuid::Uuid;

use super::models::Book;
use helpers::rpc::RpcResult;

#[tarpc::service]
pub trait BookService {
    async fn get_book(id: Uuid) -> RpcResult<Book>;
    async fn list_books() -> RpcResult<Vec<Book>>;
}
