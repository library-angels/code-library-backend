use chrono::NaiveDate;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Book {
    pub id: i32,
    /// E.g. `SE20`, `STS5`
    code_identifier: String,
    /// International Standard Book Number (https://en.wikipedia.org/wiki/International_Standard_Book_Number)
    isbn: String,
    /// International Standard Serial Number (https://en.wikipedia.org/wiki/International_Standard_Serial_Number)
    issn: Option<String>,
    release_date: NaiveDate,
    subtitle: Option<String>,
    title: String,

    #[serde(skip)]
    category_id: i32,
    #[serde(skip)]
    language_id: i32,
    #[serde(skip)]
    publisher_id: i32,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Language {
    pub id: i32,
    /// ISO 639-2/B (https://en.wikipedia.org/wiki/ISO_639-2)
    pub iso_code: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub isni: Option<String>,
    pub orcid: Option<String>,
    pub oclc: Option<i32>,
}
#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Series {
    pub id: i32,
    pub publisher_id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct SubjectArea {
    pub id: i32,
    pub name: String,
}
