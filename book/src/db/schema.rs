table! {
    books (id) {
        id -> Int4,
        code_identifier -> Varchar,
        isbn -> Varchar,
        issn -> Nullable<Varchar>,
        release_date -> Date,
        subtitle -> Nullable<Varchar>,
        title -> Varchar,
        category_id -> Int4,
        language_id -> Int4,
        publisher_id -> Int4,
    }
}

table! {
    books_authors (id) {
        id -> Int4,
        book_id -> Int4,
        person_id -> Int4,
    }
}

table! {
    books_series (id) {
        id -> Int4,
        book_id -> Int4,
        series_id -> Int4,
    }
}

table! {
    books_subject_areas (id) {
        id -> Int4,
        book_id -> Int4,
        subject_area_id -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    languages (id) {
        id -> Int4,
        iso_code -> Varchar,
        name -> Varchar,
    }
}

table! {
    persons (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        isni -> Nullable<Varchar>,
        orcid -> Nullable<Varchar>,
        oclc -> Nullable<Int4>,
    }
}

table! {
    publishers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    series (id) {
        id -> Int4,
        publisher_id -> Int4,
        name -> Varchar,
    }
}

table! {
    subject_areas (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(books -> categories (category_id));
joinable!(books -> languages (language_id));
joinable!(books -> publishers (publisher_id));
joinable!(books_authors -> books (book_id));
joinable!(books_authors -> persons (person_id));
joinable!(books_series -> books (book_id));
joinable!(books_series -> series (series_id));
joinable!(books_subject_areas -> books (book_id));
joinable!(books_subject_areas -> subject_areas (subject_area_id));
joinable!(series -> publishers (publisher_id));

allow_tables_to_appear_in_same_query!(
    books,
    books_authors,
    books_series,
    books_subject_areas,
    categories,
    languages,
    persons,
    publishers,
    series,
    subject_areas,
);
