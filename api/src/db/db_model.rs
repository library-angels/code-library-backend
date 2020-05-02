#![allow(unused)]
#![allow(clippy::all)]

use super::schema::*;
use chrono::naive::{NaiveDate, NaiveDateTime};
use diesel::{
    pg::{data_types::PgInterval, PgConnection},
    prelude::{Queryable, *},
};
use serde::{
    self,
    ser::{Serialize, SerializeStruct, Serializer},
};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct Book {
    pub id: i32,
    pub isbn_13: Option<String>,
    pub issn: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub cover: Option<i32>,
    pub edition: Option<i32>,
    pub release_year: Option<i16>,
    pub pages: Option<i32>,
    pub code_identifier: Option<String>,
    pub publisher_id: i32,
    pub designation_id: i32,
    pub series_id: Option<i32>,
    pub language_id: i32,
    pub physical_size_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct BooksAuthor {
    pub id: i32,
    pub book_id: i32,
    pub person_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct BooksEditor {
    pub id: i32,
    pub book_id: i32,
    pub person_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct BooksSubjectArea {
    pub id: i32,
    pub book_id: i32,
    pub subject_area_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct BooksTag {
    pub id: i32,
    pub book_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Debug, PartialEq)]
#[table_name = "copies"]
pub struct Copy {
    pub id: i32,
    pub book_id: i32,
    pub status_id: i32,
    pub code_identifier_copy_id: Option<i32>,
    pub date_added: NaiveDate,
}

#[derive(Queryable, Debug)]
pub struct CopiesUsersActive {
    pub id: i32,
    pub copy_id: i32,
    pub user_id: i32,
    pub borrow_from: NaiveDate,
    pub borrow_to: NaiveDate,
}

#[derive(Queryable, Debug)]
pub struct CopiesUsersHistory {
    pub id: i32,
    pub copy_id: i32,
    pub user_id: i32,
    pub borrow_start: NaiveDate,
    pub borrow_end: NaiveDate,
}

#[derive(Queryable, Debug)]
pub struct CopiesUsersReserved {
    pub id: i32,
    pub copy_id: i32,
    pub user_id: i32,
    pub duration: PgInterval,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct Designation {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct Language {
    pub id: i32,
    pub iso_code: String,
    pub language_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct Person {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub isni: Option<String>,
    pub orcid: Option<String>,
    pub oclc: Option<String>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct PhysicalSize {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub role_name: String,
    pub access_manage_books: bool,
    pub access_manage_roles: bool,
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Debug, PartialEq)]
#[table_name = "series"]
pub struct Series {
    pub id: i32,
    pub publisher_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Queryable, Debug, PartialEq)]
#[table_name = "statuses"]
pub struct Status {
    pub id: i32,
    pub name: String,
    pub available: bool,
    pub bookable: bool,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct SubjectArea {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Queryable, Debug, PartialEq)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub sub: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub picture: Option<Vec<u8>>,
    pub oauth_access_token: String,
    pub oauth_access_token_valid: NaiveDateTime,
    pub oauth_refresh_token: String,
    pub oauth_refresh_token_valid: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct UsersRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}
