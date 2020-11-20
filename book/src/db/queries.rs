use diesel::prelude::*;

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
