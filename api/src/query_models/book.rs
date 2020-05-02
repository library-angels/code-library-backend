use crate::db::schema::books;
use chrono::prelude::*;
use chrono::Utc;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BookQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub publisher_id: Option<i32>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "books"]
pub struct NewBookQuery {
    pub title: String,
    pub description: String,
    pub release_year: Option<i16>,
    pub publisher_id: i32,
    pub designation_id: i32,
    pub language_id: i32,
    pub physical_size_id: i32,
}
#[derive(Insertable, Queryable, Serialize, Deserialize, AsChangeset, Clone)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub description: String,
    pub release_year: i16,
    pub publisher_id: i32,
    pub designation_id: i32,
    pub language_id: i32,
    pub physical_size_id: i32,
}

impl From<NewBookQuery> for NewBook {
    fn from(query: NewBookQuery) -> Self {
        Self {
            title: query.title,
            description: query.description,
            release_year: query
                .release_year
                .unwrap_or_else(|| Utc::today().year() as i16),
            publisher_id: query.publisher_id,
            designation_id: query.designation_id,
            language_id: query.language_id,
            physical_size_id: query.physical_size_id,
        }
    }
}
