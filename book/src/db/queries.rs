use diesel::expression::dsl::any;
use diesel::prelude::*;

use helpers::db::pagination::Paginate;

use super::get_conn;
use super::models::*;
use super::schema::*;

type RawBookAndArgs = (Book, Category, Language, Publisher);

pub fn get_book_by_id(book_id: i32) -> QueryResult<RawBookAndArgs> {
    books::table
        .find(book_id)
        .inner_join(categories::table)
        .inner_join(languages::table)
        .inner_join(publishers::table)
        .get_result(&get_conn())
}

pub fn get_book_authors(book_id: i32) -> QueryResult<Vec<Person>> {
    books_authors::table
        .filter(books_authors::book_id.eq(book_id))
        .inner_join(persons::table)
        .select(persons::all_columns)
        .get_results(&get_conn())
}

pub fn get_book_series(book_id: i32) -> QueryResult<Option<Series>> {
    books_series::table
        .filter(books_series::book_id.eq(book_id))
        .inner_join(series::table)
        .select(series::all_columns)
        .get_result(&get_conn())
        .optional()
}

pub fn get_book_subject_areas(book_id: i32) -> QueryResult<Vec<SubjectArea>> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(book_id))
        .inner_join(subject_areas::table)
        .select(subject_areas::all_columns)
        .get_results(&get_conn())
}

pub fn list_books(page: i64, page_size: i64) -> QueryResult<(Vec<RawBookAndArgs>, i64)> {
    books::table
        .inner_join(categories::table)
        .inner_join(languages::table)
        .inner_join(publishers::table)
        .paginate(page)
        .per_page(page_size)
        .load_and_count_pages(&get_conn())
}

/// Get all authors for any of the books.
///
/// Returns a `Vec<(i32, Person)>`, where the `i32` is the `book_id`.
pub fn get_authors_of_book_list(book_ids: &[i32]) -> QueryResult<Vec<(i32, Person)>> {
    books_authors::table
        .filter(books_authors::book_id.eq(any(book_ids)))
        .inner_join(persons::table)
        .select((books_authors::book_id, persons::all_columns))
        .get_results(&get_conn())
}

/// Get all series for any of the books.
///
/// Returns a `Vec<(i32, Series)>`, where the `i32` is the `book_id`.
pub fn get_series_of_book_list(book_ids: &[i32]) -> QueryResult<Vec<(i32, Series)>> {
    books_series::table
        .filter(books_series::book_id.eq(any(book_ids)))
        .inner_join(series::table)
        .select((books_series::book_id, series::all_columns))
        .get_results(&get_conn())
}

/// Get all subject areas for any of the books.
///
/// Returns a `Vec<(i32, SubjectArea)>`, where the `i32` is the `book_id`.
pub fn get_subject_areas_of_book_list(book_ids: &[i32]) -> QueryResult<Vec<(i32, SubjectArea)>> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(any(book_ids)))
        .inner_join(subject_areas::table)
        .select((books_subject_areas::book_id, subject_areas::all_columns))
        .get_results(&get_conn())
}
