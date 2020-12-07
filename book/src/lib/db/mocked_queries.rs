use diesel::result::{Error, QueryResult};

use super::models::*;
use super::DbConn;
use crate::test_data::*;

pub type RawBookAndArgs = (Book, Category, Language, Publisher);

type ListResult<T> = QueryResult<Vec<(i32, T)>>;
type PageResult<T> = QueryResult<(Vec<T>, i64)>;
type VecResult<T> = QueryResult<Vec<T>>;

pub fn get_book_by_id(book_id: i32, _: &DbConn) -> QueryResult<RawBookAndArgs> {
    match book_id {
        1 => Ok(raw_book_and_args_1()),
        _ => not_found(),
    }
}

pub fn get_book_authors(book_id: i32, _: &DbConn) -> VecResult<Person> {
    match book_id {
        1 => Ok(authors_1()),
        _ => not_found(),
    }
}

pub fn get_book_series(book_id: i32, _: &DbConn) -> QueryResult<Option<Series>> {
    match book_id {
        1 => Ok(series_1()),
        _ => not_found(),
    }
}

pub fn get_book_subject_areas(book_id: i32, _: &DbConn) -> VecResult<SubjectArea> {
    match book_id {
        1 => Ok(subject_areas_1()),
        _ => not_found(),
    }
}

pub fn list_authors_of_books(_book_ids: &[i32], _: &DbConn) -> ListResult<Person> {
    unimplemented!()
}

pub fn list_books(_page: i64, _page_size: i64, _: &DbConn) -> PageResult<RawBookAndArgs> {
    unimplemented!()
}

pub fn list_series_of_books(_book_ids: &[i32], _: &DbConn) -> ListResult<Series> {
    unimplemented!()
}

pub fn list_subject_areas_of_books(_book_ids: &[i32], _: &DbConn) -> ListResult<SubjectArea> {
    unimplemented!()
}

fn not_found<T>() -> QueryResult<T> {
    Err(Error::NotFound)
}
