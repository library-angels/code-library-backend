use chrono::NaiveDate;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct Book {
    pub id: i32,
    /// E.g. `SE20`, `STS5`
    pub code_identifier: String,
    /// International Standard Book Number (https://en.wikipedia.org/wiki/International_Standard_Book_Number)
    pub isbn: String,
    /// International Standard Serial Number (https://en.wikipedia.org/wiki/International_Standard_Serial_Number)
    pub issn: Option<String>,
    pub release_date: NaiveDate,
    pub subtitle: Option<String>,
    pub title: String,

    #[serde(skip)]
    pub category_id: i32,
    #[serde(skip)]
    pub language_id: i32,
    #[serde(skip)]
    pub publisher_id: i32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct Language {
    pub id: i32,
    /// ISO 639-2/B (https://en.wikipedia.org/wiki/ISO_639-2)
    pub iso_code: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct Person {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub isni: Option<String>,
    pub orcid: Option<String>,
    pub oclc: Option<i32>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct Series {
    pub id: i32,
    pub publisher_id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Queryable, Serialize)]
pub struct SubjectArea {
    pub id: i32,
    pub name: String,
}
