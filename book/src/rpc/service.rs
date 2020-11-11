use super::models::Book;

pub use helpers::rpc::RpcResult;

#[tarpc::service]
pub trait BookService {
    async fn get_book(book_id: u32) -> RpcResult<Book>;
}
