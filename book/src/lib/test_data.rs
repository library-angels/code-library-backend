use chrono::NaiveDate;
use diesel::prelude::*;

use crate::db::DbConn;
use crate::models::*;

pub fn insert_data(db_conn: &DbConn) -> QueryResult<()> {
    let queries = vec![
            "INSERT INTO persons (first_name, last_name) VALUES ('Ada', 'Lovelace');",
            "INSERT INTO persons (first_name, last_name) VALUES ('Grace', 'Hopper');",
            "INSERT INTO persons (first_name, last_name) VALUES ('Mary Kenneth', 'Keller');",
            "INSERT INTO publishers (name) VALUES ('Nostarch Press');",
            "INSERT INTO publishers (name) VALUES ('MIT Press');",
            "INSERT INTO books (code_identifier, isbn, release_date, title, subtitle, category_id, language_id, publisher_id) VALUES ('SE001', '1234567891011', '1900-01-01', 'Rust Book', 'Why you should learn it', 1, 1, 1);",
            "INSERT INTO books (code_identifier, isbn, release_date, title, subtitle, category_id, language_id, publisher_id) VALUES ('SE002', '1234567891012', '2000-01-01', 'Python Book', DEFAULT, 1, 2, 2);",
            "INSERT INTO books (code_identifier, isbn, release_date, title, subtitle, category_id, language_id, publisher_id) VALUES ('SE003', '1234567891013', '2100-01-01', 'Javascript Book', 'Why you shouldn''t learn it', 1, 3, 1);",
            "INSERT INTO books_authors (book_id, person_id) VALUES (1, 1), (1, 2), (2, 2), (3, 1), (3, 2);"
        ];
    for query in queries {
        diesel::sql_query(query).execute(db_conn)?;
    }
    Ok(())
}

pub fn book_1() -> Book {
    Book::new(
        RawBook {
            id: 1,
            code_identifier: "SE001".to_string(),
            isbn: "1234567891011".to_string(),
            issn: None,
            release_date: NaiveDate::from_ymd(1900, 1, 1),
            subtitle: Some("Why you should learn it".to_string()),
            title: "Rust Book".to_string(),
            category_id: 0,
            language_id: 0,
            publisher_id: 0,
        },
        Category {
            id: 1,
            name: "STS".to_string(),
        },
        Language {
            id: 1,
            iso_code: "aar".to_string(),
            name: "Afar".to_string(),
        },
        Publisher {
            id: 1,
            name: "Nostarch Press".to_string(),
        },
        None,
        vec![
            Person {
                id: 1,
                first_name: "Ada".to_string(),
                last_name: "Lovelace".to_string(),
                isni: None,
                orcid: None,
                oclc: None,
            },
            Person {
                id: 2,
                first_name: "Grace".to_string(),
                last_name: "Hopper".to_string(),
                isni: None,
                orcid: None,
                oclc: None,
            },
        ],
        vec![],
    )
}
