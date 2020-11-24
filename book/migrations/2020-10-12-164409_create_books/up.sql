-- 
-- TABLES
--
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE persons (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    isni VARCHAR(16) UNIQUE,
    orcid VARCHAR(16) UNIQUE,
    oclc INTEGER UNIQUE
);

CREATE TABLE publishers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE series (
    id SERIAL PRIMARY KEY,
    publisher_id INTEGER NOT NULL REFERENCES publishers (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    name VARCHAR(100) NOT NULL,
    UNIQUE(publisher_id, name)
);

CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    iso_code VARCHAR(3) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE subject_areas (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    -- 
    code_identifier VARCHAR(100) UNIQUE NOT NULL,
    isbn VARCHAR(13) UNIQUE NOT NULL,
    issn VARCHAR(8),
    release_date DATE NOT NULL,
    subtitle VARCHAR(500),
    title VARCHAR(100) NOT NULL,
    -- 
    category_id INTEGER NOT NULL REFERENCES categories (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    language_id INTEGER NOT NULL REFERENCES languages (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    publisher_id INTEGER NOT NULL REFERENCES publishers (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE books_subject_areas (
    id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE CASCADE,
    subject_area_id INTEGER NOT NULL REFERENCES subject_areas (id) ON UPDATE CASCADE ON DELETE CASCADE,
    UNIQUE(book_id, subject_area_id)
);

CREATE TABLE books_authors (
    id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    person_id INTEGER NOT NULL REFERENCES persons (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(book_id, person_id)
);

CREATE TABLE books_series (
    id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books ON UPDATE CASCADE ON DELETE CASCADE,
    series_id INTEGER NOT NULL REFERENCES series ON UPDATE CASCADE ON DELETE CASCADE,
    -- `book_id` set `UNIQUE`, cause we assume that one book will only be part of one series
    UNIQUE(book_id)
);
