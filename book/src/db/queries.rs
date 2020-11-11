use diesel::prelude::*;

use super::schema::*;
use super::{get_conn, models as db_models};
use crate::rpc::models as rpc_models;

pub fn get_book_by_id(book_id: i32) -> QueryResult<rpc_models::Book> {
    let (raw_book, category, language, publisher, series) = books::table
        .find(book_id)
        .inner_join(categories::table)
        .inner_join(languages::table)
        .inner_join(publishers::table)
        .inner_join(series::table)
        .get_result::<(
            db_models::Book,
            db_models::Category,
            db_models::Language,
            db_models::Publisher,
            db_models::Series,
        )>(&get_conn())?;

    Ok(rpc_models::Book::new(
        raw_book,
        category,
        language,
        publisher,
        series,
        self::get_book_authors(book_id)?,
        self::get_book_subject_areas(book_id)?,
    ))
}

fn get_book_authors(book_id: i32) -> QueryResult<Vec<db_models::Person>> {
    books_authors::table
        .filter(books_authors::book_id.eq(book_id))
        .inner_join(persons::table)
        .select(persons::all_columns)
        .get_results::<db_models::Person>(&get_conn())
}

fn get_book_subject_areas(book_id: i32) -> QueryResult<Vec<db_models::SubjectArea>> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(book_id))
        .inner_join(subject_areas::table)
        .select(subject_areas::all_columns)
        .get_results::<db_models::SubjectArea>(&get_conn())
}
