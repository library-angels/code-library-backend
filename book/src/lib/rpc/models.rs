use serde::{Deserialize, Serialize};

pub use helpers::rpc::{Error, RpcResult};

pub use crate::db::models::{
    Book as RawBook, Category, Language, Person, Publisher, Series, SubjectArea,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Book {
    #[serde(flatten)]
    raw_book: RawBook,

    // one-to-many
    category: Category,
    language: Language,
    publisher: Publisher,
    series: Option<Series>,

    // many-to-many
    authors: Vec<Person>,
    subject_areas: Vec<SubjectArea>,
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
            raw_book,
            category,
            language,
            publisher,
            series,
            authors,
            subject_areas,
        }
    }

    pub fn id(&self) -> i32 {
        self.raw_book.id
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
