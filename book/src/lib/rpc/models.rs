use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Language {
    pub id: Uuid,
    iso_code: String,
    name: String,
}

impl From<crate::db::models::Language> for Language {
    fn from(language: crate::db::models::Language) -> Self {
        Language {
            id: language.id,
            iso_code: language.iso_code,
            name: language.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub id: Uuid,
    name: String,
}

impl From<crate::db::models::Category> for Category {
    fn from(category: crate::db::models::Category) -> Self {
        Category {
            id: category.id,
            name: category.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Publisher {
    pub id: Uuid,
    name: String,
}

impl From<crate::db::models::Publisher> for Publisher {
    fn from(publisher: crate::db::models::Publisher) -> Self {
        Publisher {
            id: publisher.id,
            name: publisher.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Series {
    pub id: Uuid,
    publisher_id: Uuid,
    name: String,
}

impl From<crate::db::models::Series> for Series {
    fn from(series: crate::db::models::Series) -> Self {
        Series {
            id: series.id,
            publisher_id: series.publisher_id,
            name: series.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubjectArea {
    pub id: Uuid,
    name: String,
}

impl From<crate::db::models::SubjectArea> for SubjectArea {
    fn from(subject_area: crate::db::models::SubjectArea) -> Self {
        SubjectArea {
            id: subject_area.id,
            name: subject_area.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: Uuid,
    name: String,
}

impl From<crate::db::models::Tag> for Tag {
    fn from(tag: crate::db::models::Tag) -> Self {
        Tag {
            id: tag.id,
            name: tag.name,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    pub id: Uuid,
    first_name: String,
    last_name: String,
    date_of_birth: Option<NaiveDate>,
    isni: Option<String>,
    orcid: Option<String>,
    oclc: Option<i32>,
}

impl From<crate::db::models::Author> for Author {
    fn from(author: crate::db::models::Author) -> Self {
        Author {
            id: author.id,
            first_name: author.first_name,
            last_name: author.last_name,
            date_of_birth: author.date_of_birth,
            isni: author.isni,
            orcid: author.orcid,
            oclc: author.oclc,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Editor {
    pub id: Uuid,
    first_name: String,
    last_name: String,
    date_of_birth: Option<NaiveDate>,
    isni: Option<String>,
    orcid: Option<String>,
    oclc: Option<i32>,
}

impl From<crate::db::models::Editor> for Editor {
    fn from(editor: crate::db::models::Editor) -> Self {
        Editor {
            id: editor.id,
            first_name: editor.first_name,
            last_name: editor.last_name,
            date_of_birth: editor.date_of_birth,
            isni: editor.isni,
            orcid: editor.orcid,
            oclc: editor.oclc,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Copy {
    pub id: Uuid,
    pub book_id: Uuid,
    pub copy_id: i64,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

impl From<crate::db::models::Copy> for Copy {
    fn from(copy: crate::db::models::Copy) -> Self {
        Copy {
            id: copy.id,
            book_id: copy.book_id,
            copy_id: copy.copy_id,
            created_at: copy.created_at,
            created_by: copy.created_by,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: Uuid,
    code_identifier: i32,
    isbn: Option<String>,
    issn: Option<String>,
    release_year: i16,
    edition: Option<i32>,
    pages: Option<i32>,
    title: String,
    subtitle: Option<String>,
    description: Option<String>,
}

impl From<crate::db::models::Book> for Book {
    fn from(book: crate::db::models::Book) -> Self {
        Book {
            id: book.id,
            code_identifier: book.code_identifier,
            isbn: book.isbn,
            issn: book.issn,
            release_year: book.release_year,
            edition: book.edition,
            pages: book.pages,
            title: book.title,
            subtitle: book.subtitle,
            description: book.description,
        }
    }
}
