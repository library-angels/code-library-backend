use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

pub use helpers::rpc::{Error, RpcResult};

pub(crate) use crate::db::models::Book as RawBook;
pub use crate::db::models::*;

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
    pub category: Category,
    pub language: Language,
    pub publisher: Publisher,
    pub series: Option<Series>,

    // many-to-many
    pub authors: Vec<Person>,
    pub subject_areas: Vec<SubjectArea>,
}

impl Book {
    pub fn new(
        raw_book: RawBook,
        category: Category,
        language: Language,
        publisher: Publisher,
        series: Option<Series>,
        authors: Vec<Person>,
        subject_areas: Vec<SubjectArea>,
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

    pub fn push_author(&mut self, a: Person) {
        self.authors.push(a);
    }

    pub fn push_subject_area(&mut self, s: SubjectArea) {
        self.subject_areas.push(s);
    }

    pub fn set_series(&mut self, s: Series) {
        self.series = Some(s);
    }
}
