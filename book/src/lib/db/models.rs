use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Language {
    pub id: Uuid,
    pub iso_code: String,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Publisher {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Series {
    pub id: Uuid,
    pub publisher_id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SubjectArea {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Author {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub isni: Option<String>,
    pub orcid: Option<String>,
    pub oclc: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Editor {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub isni: Option<String>,
    pub orcid: Option<String>,
    pub oclc: Option<i32>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Copy {
    pub id: Uuid,
    pub book_id: Uuid,
    pub copy_id: i64,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Book {
    pub id: Uuid,
    pub code_identifier: i32,
    pub isbn: Option<String>,
    pub issn: Option<String>,
    pub release_year: i16,
    pub edition: Option<i32>,
    pub pages: Option<i32>,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
}
