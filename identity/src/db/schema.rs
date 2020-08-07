table! {
    books (id) {
        id -> Int4,
        isbn_13 -> Nullable<Varchar>,
        issn -> Nullable<Varchar>,
        title -> Varchar,
        subtitle -> Nullable<Varchar>,
        description -> Nullable<Text>,
        cover -> Nullable<Int4>,
        edition -> Nullable<Int4>,
        release_year -> Nullable<Int2>,
        pages -> Nullable<Int4>,
        code_identifier -> Nullable<Varchar>,
        publisher_id -> Int4,
        category_id -> Int4,
        series_id -> Nullable<Int4>,
        language_id -> Int4,
        physical_size_id -> Int4,
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
    books_editors (id) {
        id -> Int4,
        book_id -> Int4,
        person_id -> Int4,
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
    books_tags (id) {
        id -> Int4,
        book_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    copies (id) {
        id -> Int4,
        book_id -> Int4,
        status_id -> Int4,
        code_identifier_copy_id -> Nullable<Int4>,
        date_added -> Date,
    }
}

table! {
    copies_users_active (id) {
        id -> Int4,
        copy_id -> Int4,
        user_id -> Int4,
        borrow_from -> Date,
        borrow_to -> Date,
    }
}

table! {
    copies_users_history (id) {
        id -> Int4,
        copy_id -> Int4,
        user_id -> Int4,
        borrow_start -> Date,
        borrow_end -> Date,
    }
}

table! {
    copies_users_reserved (id) {
        id -> Int4,
        copy_id -> Int4,
        user_id -> Int4,
        duration -> Interval,
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
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        date_of_birth -> Nullable<Date>,
        isni -> Nullable<Varchar>,
        orcid -> Nullable<Varchar>,
        oclc -> Nullable<Varchar>,
    }
}

table! {
    physical_sizes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    publishers (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        access_manage_books -> Bool,
        access_manage_roles -> Bool,
    }
}

table! {
    series (id) {
        id -> Int4,
        publisher_id -> Int4,
        name -> Nullable<Varchar>,
    }
}

table! {
    statuses (id) {
        id -> Int4,
        name -> Varchar,
        available -> Bool,
        bookable -> Bool,
    }
}

table! {
    subject_areas (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        sub -> Varchar,
        email -> Varchar,
        given_name -> Varchar,
        family_name -> Varchar,
        picture -> Varchar,
        oauth_access_token -> Varchar,
        oauth_access_token_valid -> Timestamp,
        oauth_refresh_token -> Varchar,
        active -> Bool,
    }
}

table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

joinable!(books -> categories (category_id));
joinable!(books -> languages (language_id));
joinable!(books -> physical_sizes (physical_size_id));
joinable!(books -> publishers (publisher_id));
joinable!(books -> series (series_id));
joinable!(books_authors -> books (book_id));
joinable!(books_authors -> persons (person_id));
joinable!(books_editors -> books (book_id));
joinable!(books_editors -> persons (person_id));
joinable!(books_subject_areas -> books (book_id));
joinable!(books_subject_areas -> subject_areas (subject_area_id));
joinable!(books_tags -> books (book_id));
joinable!(books_tags -> tags (tag_id));
joinable!(copies -> books (book_id));
joinable!(copies -> statuses (status_id));
joinable!(copies_users_active -> copies (copy_id));
joinable!(copies_users_active -> users (user_id));
joinable!(copies_users_history -> copies (copy_id));
joinable!(copies_users_history -> users (user_id));
joinable!(copies_users_reserved -> copies (copy_id));
joinable!(copies_users_reserved -> users (user_id));
joinable!(series -> publishers (publisher_id));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    books,
    books_authors,
    books_editors,
    books_subject_areas,
    books_tags,
    categories,
    copies,
    copies_users_active,
    copies_users_history,
    copies_users_reserved,
    languages,
    persons,
    physical_sizes,
    publishers,
    roles,
    series,
    statuses,
    subject_areas,
    tags,
    users,
    users_roles,
);
