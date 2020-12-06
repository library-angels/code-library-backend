use chrono::NaiveDate;
use diesel::prelude::*;

use crate::db::{queries::RawBookAndArgs, DbConn};
use crate::models::*;

pub fn insert_data(db_conn: &DbConn) -> QueryResult<()> {
    let queries = vec![
            "INSERT INTO persons VALUES (1, 'Ada', 'Lovelace');",
            "INSERT INTO persons VALUES (2, 'Grace', 'Hopper');",
            "INSERT INTO persons VALUES (3, 'Mary Kenneth', 'Keller');",
            "INSERT INTO publishers VALUES (1, 'Nostarch Press');",
            "INSERT INTO publishers VALUES (2, 'MIT Press');",
            "INSERT INTO books VALUES (1, 'SE001', '1234567891011', null, '1900-01-01', 'Why you should learn it', 'Rust Book', 1, 1, 1);",
            "INSERT INTO books VALUES (2, 'SE002', '1234567891012', null, '2000-01-01', DEFAULT, 'Python Book', 1, 2, 2);",
            "INSERT INTO books VALUES (3, 'SE003', '1234567891013', null, '2100-01-01', 'Why you shouldn''t learn it', 'Javascript Book', 1, 3, 1);",
            "INSERT INTO books_authors VALUES (1, 1, 1), (2, 1, 2), (3, 2, 2), (4, 3, 1), (5, 3, 2);",
            "INSERT INTO series VALUES (1, 1, 'Great Books');",
            "INSERT INTO books_series VALUES (1, 1, 1);",
            "INSERT INTO subject_areas VALUES (1, 'Software');",
            "INSERT INTO books_subject_areas VALUES (1, 1, 1);"
        ];
    for query in queries {
        diesel::sql_query(query).execute(db_conn)?;
    }
    Ok(())
}

pub fn authors_1() -> Vec<Person> {
    vec![person_1(), person_2()]
}

pub fn book_1() -> Book {
    Book::new(
        raw_book_1(),
        category_1(),
        language_1(),
        publisher_1(),
        series_1(),
        authors_1(),
        subject_areas_1(),
    )
}

pub fn category_1() -> Category {
    Category {
        id: 1,
        name: "STS".to_string(),
    }
}

pub fn language_1() -> Language {
    Language {
        id: 1,
        iso_code: "aar".to_string(),
        name: "Afar".to_string(),
    }
}

pub fn person_1() -> Person {
    Person {
        id: 1,
        first_name: "Ada".to_string(),
        last_name: "Lovelace".to_string(),
        isni: None,
        orcid: None,
        oclc: None,
    }
}
pub fn person_2() -> Person {
    Person {
        id: 2,
        first_name: "Grace".to_string(),
        last_name: "Hopper".to_string(),
        isni: None,
        orcid: None,
        oclc: None,
    }
}

pub fn publisher_1() -> Publisher {
    Publisher {
        id: 1,
        name: "Nostarch Press".to_string(),
    }
}

pub fn raw_book_1() -> RawBook {
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
    }
}

pub fn raw_book_and_args() -> RawBookAndArgs {
    (raw_book_1(), category_1(), language_1(), publisher_1())
}

pub fn series_1() -> Option<Series> {
    Some(Series {
        id: 1,
        publisher_id: 1,
        name: "Great Books".to_string(),
    })
}

pub fn subject_area_1() -> SubjectArea {
    SubjectArea {
        id: 1,
        name: "Software".to_string(),
    }
}

pub fn subject_areas_1() -> Vec<SubjectArea> {
    vec![subject_area_1()]
}
