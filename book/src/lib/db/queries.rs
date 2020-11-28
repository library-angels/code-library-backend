use diesel::expression::dsl::any;
use diesel::prelude::*;

use helpers::db::pagination::Paginate;
use helpers::ulid::Ulid;

use super::get_conn;
use super::models::*;
use super::schema::*;

type RawBookAndArgs = (Book, Category, Language, Publisher);

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
pub fn get_book_by_id(book_id: Ulid) -> QueryResult<RawBookAndArgs> {
    books::table
        .find(book_id)
        .inner_join(categories::table)
        .inner_join(languages::table)
        .inner_join(publishers::table)
        .get_result(&get_conn())
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
pub fn get_book_authors(book_id: Ulid) -> QueryResult<Vec<Person>> {
    books_authors::table
        .filter(books_authors::book_id.eq(book_id))
        .inner_join(persons::table)
        .select(persons::all_columns)
        .get_results(&get_conn())
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
pub fn get_book_series(book_id: Ulid) -> QueryResult<Option<Series>> {
    books_series::table
        .filter(books_series::book_id.eq(book_id))
        .inner_join(series::table)
        .select(series::all_columns)
        .get_result(&get_conn())
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
pub fn get_book_subject_areas(book_id: Ulid) -> QueryResult<Vec<SubjectArea>> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(book_id))
        .inner_join(subject_areas::table)
        .select(subject_areas::all_columns)
        .get_results(&get_conn())
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
pub fn list_books(page: i64, page_size: i64) -> QueryResult<(Vec<RawBookAndArgs>, i64)> {
    books::table
        .inner_join(categories::table)
        .inner_join(languages::table)
        .inner_join(publishers::table)
        .paginate(page)
        .per_page(page_size)
        .load_and_count_pages(&get_conn())
}

/// Get all authors for any of the books.
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
pub fn get_authors_of_book_list(book_ids: &[Ulid]) -> QueryResult<Vec<(Ulid, Person)>> {
    books_authors::table
        .filter(books_authors::book_id.eq(any(book_ids)))
        .inner_join(persons::table)
        .select((books_authors::book_id, persons::all_columns))
        .get_results(&get_conn())
}

/// Get all series for any of the books.
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
pub fn get_series_of_book_list(book_ids: &[Ulid]) -> QueryResult<Vec<(Ulid, Series)>> {
    books_series::table
        .filter(books_series::book_id.eq(any(book_ids)))
        .inner_join(series::table)
        .select((books_series::book_id, series::all_columns))
        .get_results(&get_conn())
}

/// Get all subject areas for any of the books.
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
pub fn get_subject_areas_of_book_list(book_ids: &[Ulid]) -> QueryResult<Vec<(Ulid, SubjectArea)>> {
    books_subject_areas::table
        .filter(books_subject_areas::book_id.eq(any(book_ids)))
        .inner_join(subject_areas::table)
        .select((books_subject_areas::book_id, subject_areas::all_columns))
        .get_results(&get_conn())
}
