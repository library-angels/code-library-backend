use std::collections::HashMap;

use tarpc::context::Context;

use super::models::{Book, RpcResult};
use super::service::BookService;
use crate::db::{queries, DbConn, DbPool};

#[derive(Clone)]
pub struct BookServer {
    db_pool: DbPool,
}

impl BookServer {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    pub fn db_conn(&self) -> DbConn {
        self.db_pool
            .get()
            .expect("Failed to retrieve connection from the pool")
    }
}

#[tarpc::server]
impl BookService for BookServer {
    async fn get_book(self, _: Context, book_id: u32) -> RpcResult<Book> {
        let book_id = book_id as i32;

        let (raw_book, category, language, publisher) =
            queries::get_book_by_id(book_id, &self.db_conn())?;

        Ok(Book::new(
            raw_book,
            category,
            language,
            publisher,
            queries::get_book_series(book_id, &self.db_conn())?,
            queries::get_book_authors(book_id, &self.db_conn())?,
            queries::get_book_subject_areas(book_id, &self.db_conn())?,
        ))
    }

    async fn list_books(
        self,
        _: Context,
        page: u32,
        page_size: u32,
    ) -> RpcResult<(Vec<Book>, u32)> {
        // get books
        let (book_list, num_pages) =
            queries::list_books(page as i64, page_size as i64, &self.db_conn())?;
        // get ids of books
        let book_ids = book_list.iter().map(|(b, ..)| b.id).collect::<Vec<_>>();

        // map book_id to book
        let mut book_map: HashMap<i32, Book> = HashMap::with_capacity(book_list.len());
        for (b, c, l, p) in book_list.into_iter() {
            let new_book = Book::new(b, c, l, p, None, Vec::new(), Vec::new());
            if let Some(err_book) = book_map.insert(new_book.id(), new_book) {
                panic!("Duplicate book id: {}", err_book.id());
            }
        }

        // get authors, series, subject_areas for all books
        let authors = queries::list_authors_of_books(&book_ids, &self.db_conn())?;
        sort_vals_into_map(&mut book_map, authors, &mut Book::push_author);
        let series = queries::list_series_of_books(&book_ids, &self.db_conn())?;
        sort_vals_into_map(&mut book_map, series, &mut Book::set_series);
        let subject_areas = queries::list_subject_areas_of_books(&book_ids, &self.db_conn())?;
        sort_vals_into_map(&mut book_map, subject_areas, &mut Book::push_subject_area);

        // disolve map into list of `Book`s (can be simplified with https://github.com/rust-lang/rust/issues/75294)
        let book_list = book_map.into_iter().map(|(_, b)| b).collect();

        Ok((book_list, num_pages as u32))
    }
}

fn sort_vals_into_map<T>(
    book_map: &mut HashMap<i32, Book>,
    vec: Vec<(i32, T)>,
    push_val_into_book: &mut impl std::ops::FnMut(&mut Book, T),
) {
    for (book_id, v) in vec.into_iter() {
        match book_map.get_mut(&book_id) {
            Some(book) => push_val_into_book(book, v),
            None => unreachable!(),
        };
    }
}
