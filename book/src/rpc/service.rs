use super::models::{Book, Error};

#[tarpc::service]
pub trait BookService {
    async fn get_book(book_id: u32) -> Result<Book, Error>;
}
