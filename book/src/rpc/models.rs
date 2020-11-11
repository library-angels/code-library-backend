use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::db::models as db_models;

pub use helpers::rpc::{Error, RpcResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: i32,
    /// E.g. SE20, STS5
    pub code_identifier: String,
    /// International Standard Book Number (https://en.wikipedia.org/wiki/International_Standard_Book_Number)
    pub isbn: String,
    /// International Standard Serial Number (https://en.wikipedia.org/wiki/International_Standard_Serial_Number)
    pub issn: Option<String>,
    pub release_date: NaiveDate,
    pub subtitle: Option<String>,
    pub title: String,

    // one-to-many
    pub category: db_models::Category,
    pub language: db_models::Language,
    pub publisher: db_models::Publisher,
    pub series: db_models::Series,

    // many-to-many
    pub authors: Vec<db_models::Person>,
    pub subject_areas: Vec<db_models::SubjectArea>,
}

impl Book {
    pub fn new(
        raw_book: db_models::Book,
        category: db_models::Category,
        language: db_models::Language,
        publisher: db_models::Publisher,
        series: db_models::Series,
        authors: Vec<db_models::Person>,
        subject_areas: Vec<db_models::SubjectArea>,
    ) -> Self {
        Self {
            id: raw_book.id,
            code_identifier: raw_book.code_identifier,
            isbn: raw_book.isbn,
            issn: raw_book.issn,
            release_date: raw_book.release_date,
            subtitle: raw_book.subtitle,
            title: raw_book.title,
            category,
            language,
            publisher,
            series,
            authors,
            subject_areas,
        }
    }
}
