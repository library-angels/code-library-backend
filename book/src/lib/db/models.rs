use chrono::NaiveDate;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Queryable)]
pub struct Book {
    pub id: Uuid,
    /// E.g. `SE20`, `STS5`
    pub code_identifier: String,
    /// International Standard Book Number (https://en.wikipedia.org/wiki/International_Standard_Book_Number)
    pub isbn: String,
    /// International Standard Serial Number (https://en.wikipedia.org/wiki/International_Standard_Serial_Number)
    pub issn: Option<String>,
    pub release_date: NaiveDate,
    pub subtitle: Option<String>,
    pub title: String,

    pub category_id: i32,
    pub language_id: i32,
    pub publisher_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Language {
    pub id: i32,
    pub name: String,
    /// ISO 639-2/B (https://en.wikipedia.org/wiki/ISO_639-2)
    pub iso_code: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Person {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub isni: Option<String>,
    pub orcid: Option<String>,
    pub oclc: Option<i32>,
}
#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Publisher {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct Series {
    pub id: Uuid,
    pub publisher_id: Uuid,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct SubjectArea {
    pub id: Uuid,
    pub name: String,
}
