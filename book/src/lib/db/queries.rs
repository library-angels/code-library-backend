use diesel::expression::dsl::any;
use diesel::prelude::*;

use helpers::db::pagination::Paginate;

use super::models::*;
use super::schema::*;
use super::DbConn;

pub type RawBookAndArgs = (Book, Category, Language, Publisher);

type ListResult<T> = QueryResult<Vec<(i32, T)>>;
type PageResult<T> = QueryResult<(Vec<T>, i64)>;
type VecResult<T> = QueryResult<Vec<T>>;

/// Get category, language, publisher of a book and the book itself.
///
/// ```sql
/// SELECT "books"."id",
///     "books"."code_identifier",
///     "books"."isbn",
///     "books"."issn",
///     "books"."release_date",
///     "books"."subtitle",
///     "books"."title",
///     "books"."category_id",
///     "books"."language_id",
///     "books"."publisher_id",
///     "categories"."id",
///     "categories"."name",
///     "languages"."id",
///     "languages"."iso_code",
///     "languages"."name",
///     "publishers"."id",
///     "publishers"."name"
/// FROM (((
///             "books" AS "b"
///             INNER JOIN "categories" AS "c" ON "books"."category_id" = "categories"."id"
///         )
///         INNER JOIN "languages" AS "l" ON "books"."language_id" = "languages"."id"
///     )
///     INNER JOIN "publishers" AS "p" ON "books"."publisher_id" = "publishers"."id"
/// )
/// WHERE "books"."id" = $1 -- binds: [1]
/// ```
pub fn get_book_by_id(book_id: i32, conn: &DbConn) -> QueryResult<RawBookAndArgs> {
    books::table
        .find(book_id)
        .inner_join(categories::table)
        .inner_join(languages::table)
        .inner_join(publishers::table)
        .get_result(conn)
}

/// Get all authors of a single book.
///
/// ```sql
/// SELECT "persons"."id",
/// "persons"."first_name",
/// "persons"."last_name",
/// "persons"."isni",
/// "persons"."orcid",
/// "persons"."oclc"
/// FROM (
///     "books_authors"
///     INNER JOIN "persons" ON "books_authors"."person_id" = "persons"."id"
/// )
/// WHERE "books_authors"."book_id" = $1 -- binds: [1]
/// ```
pub fn get_book_authors(book_id: i32, conn: &DbConn) -> VecResult<Person> {
    books_authors::table
        .filter(books_authors::book_id.eq(book_id))
        .inner_join(persons::table)
        .select(persons::all_columns)
        .get_results(conn)
}

/// Get the series of a single book, if there is one.
///
/// ```sql
/// SELECT "series"."id",
///     "series"."publisher_id",
///     "series"."name"
/// FROM (
///         "books_series"
///         INNER JOIN "series" ON "books_series"."series_id" = "series"."id"
///     )
/// WHERE "books_series"."book_id" = $1 -- binds: [1]
/// ```
pub fn get_book_series(book_id: i32, conn: &DbConn) -> QueryResult<Option<Series>> {
    books_series::table
        .filter(books_series::book_id.eq(book_id))
        .inner_join(series::table)
        .select(series::all_columns)
        .get_result(conn)
        .optional()
}

/// Get all subject areas of a single book.
///
/// ```sql
/// SELECT "subject_areas"."id",
///     "subject_areas"."name"
/// FROM (
///         "books_subject_areas"
///         INNER JOIN "subject_areas" ON "books_subject_areas"."subject_area_id" = "subject_areas"."id"
///     )
/// WHERE "books_subject_areas"."book_id" = $1 -- binds: [1]
/// ```
pub fn get_book_subject_areas(book_id: i32, conn: &DbConn) -> VecResult<SubjectArea> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(book_id))
        .inner_join(subject_areas::table)
        .select(subject_areas::all_columns)
        .get_results(conn)
}

/// List all authors for any of the books.
///
/// Returns a `Vec<(i32, Person)>`, where the `i32` is the `book_id`.
///
/// ```sql
/// SELECT "books_authors"."book_id",
///     "persons"."id",
///     "persons"."first_name",
///     "persons"."last_name",
///     "persons"."isni",
///     "persons"."orcid",
///     "persons"."oclc"
/// FROM (
///         "books_authors"
///         INNER JOIN "persons" ON "books_authors"."person_id" = "persons"."id"
///     )
/// WHERE "books_authors"."book_id" = ANY($1) -- binds: [[1, 2, 3]]
/// ```
pub fn list_authors_of_books(book_ids: &[i32], conn: &DbConn) -> ListResult<Person> {
    books_authors::table
        .filter(books_authors::book_id.eq(any(book_ids)))
        .inner_join(persons::table)
        .select((books_authors::book_id, persons::all_columns))
        .get_results(conn)
}

/// List (one page of) all books.
///
/// ```sql
/// SELECT *,
///     COUNT(*) OVER ()
/// FROM (
///         SELECT "books"."id",
///             "books"."code_identifier",
///             "books"."isbn",
///             "books"."issn",
///             "books"."release_date",
///             "books"."subtitle",
///             "books"."title",
///             "books"."category_id",
///             "books"."language_id",
///             "books"."publisher_id",
///             "categories"."id",
///             "categories"."name",
///             "languages"."id",
///             "languages"."iso_code",
///             "languages"."name",
///             "publishers"."id",
///             "publishers"."name"
///         FROM (((
///                     "books"
///                     INNER JOIN "categories" ON "books"."category_id" = "categories"."id"
///                 )
///                 INNER JOIN "languages" ON "books"."language_id" = "languages"."id"
///             )
///             INNER JOIN "publishers" ON "books"."publisher_id" = "publishers"."id"
///         )
///     ) t
/// LIMIT $1 OFFSET $2 -- binds: [10, 0]
/// ```
pub fn list_books(
    page: i64,
    page_size: i64,
    category: Option<String>,
    conn: &DbConn,
) -> PageResult<RawBookAndArgs> {
    match category {
        Some(c) => books::table
            .inner_join(categories::table)
            .filter(categories::name.eq(c))
            .inner_join(languages::table)
            .inner_join(publishers::table)
            .paginate(page)
            .per_page(page_size)
            .load_and_count_pages(conn),
        None => books::table
            .inner_join(categories::table)
            .inner_join(languages::table)
            .inner_join(publishers::table)
            .paginate(page)
            .per_page(page_size)
            .load_and_count_pages(conn),
    }
}

/// List all series for any of the books.
///
/// Returns a `Vec<(i32, Series)>`, where the `i32` is the `book_id`.
///
/// ```sql
/// SELECT "books_series"."book_id",
///     "series"."id",
///     "series"."publisher_id",
///     "series"."name"
/// FROM (
///         "books_series"
///         INNER JOIN "series" ON "books_series"."series_id" = "series"."id"
///     )
/// WHERE "books_series"."book_id" = ANY($1) -- binds: [[1, 2, 3]]
/// ```
pub fn list_series_of_books(book_ids: &[i32], conn: &DbConn) -> ListResult<Series> {
    books_series::table
        .filter(books_series::book_id.eq(any(book_ids)))
        .inner_join(series::table)
        .select((books_series::book_id, series::all_columns))
        .get_results(conn)
}

/// List all subject areas for any of the books.
///
/// Returns a `Vec<(i32, SubjectArea)>`, where the `i32` is the `book_id`.
///
/// ```sql
/// SELECT "books_subject_areas"."book_id",
///     "subject_areas"."id",
///     "subject_areas"."name"
/// FROM (
///         "books_subject_areas"
///         INNER JOIN "subject_areas" ON "books_subject_areas"."subject_area_id" = "subject_areas"."id"
///     )
/// WHERE "books_subject_areas"."book_id" = ANY($1) -- binds: [[1, 2, 3]]
/// ```
pub fn list_subject_areas_of_books(book_ids: &[i32], conn: &DbConn) -> ListResult<SubjectArea> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(any(book_ids)))
        .inner_join(subject_areas::table)
        .select((books_subject_areas::book_id, subject_areas::all_columns))
        .get_results(conn)
}
