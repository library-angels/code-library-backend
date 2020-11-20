use super::models::{Book, RpcResult};

#[tarpc::service]
pub trait BookService {
    async fn get_book(book_id: u32) -> RpcResult<Book>;
    async fn list_books(page: u32, page_size: u32) -> RpcResult<(Vec<Book>, u32)>;
}
