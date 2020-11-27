CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE languages (
    id SERIAL PRIMARY KEY,
    iso_code VARCHAR(3) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL
);

CREATE TABLE persons (
    id UUID PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    isni VARCHAR(16) UNIQUE,
    orcid VARCHAR(16) UNIQUE,
    oclc INTEGER UNIQUE
);

CREATE TABLE publishers (
    id UUID PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE series (
    id UUID PRIMARY KEY,
    publisher_id UUID NOT NULL REFERENCES publishers ON UPDATE CASCADE ON DELETE RESTRICT,
    name VARCHAR(100) NOT NULL,
    UNIQUE(publisher_id, name)
);

CREATE TABLE subject_areas (
    id UUID PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

CREATE TABLE books (
    id UUID PRIMARY KEY,
    -- 
    code_identifier VARCHAR(100) UNIQUE NOT NULL,
    isbn VARCHAR(13) UNIQUE NOT NULL,
    issn VARCHAR(8),
    release_date DATE NOT NULL,
    subtitle VARCHAR(500),
    title VARCHAR(100) NOT NULL,
    -- 
    category_id INT NOT NULL REFERENCES categories ON UPDATE CASCADE ON DELETE RESTRICT,
    language_id INT NOT NULL REFERENCES languages ON UPDATE CASCADE ON DELETE RESTRICT,
    publisher_id UUID NOT NULL REFERENCES publishers ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE books_authors (
    id UUID PRIMARY KEY,
    book_id UUID NOT NULL REFERENCES books ON UPDATE CASCADE ON DELETE RESTRICT,
    person_id UUID NOT NULL REFERENCES persons ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(book_id, person_id)
);

CREATE TABLE books_series (
    id UUID PRIMARY KEY,
    book_id UUID NOT NULL REFERENCES books ON UPDATE CASCADE ON DELETE CASCADE,
    series_id UUID NOT NULL REFERENCES series ON UPDATE CASCADE ON DELETE CASCADE,
    -- `book_id` set `UNIQUE`, cause we assume that one book will only be part of one series
    UNIQUE(book_id)
);

CREATE TABLE books_subject_areas (
    id UUID PRIMARY KEY,
    book_id UUID NOT NULL REFERENCES books ON UPDATE CASCADE ON DELETE CASCADE,
    subject_area_id UUID NOT NULL REFERENCES subject_areas ON UPDATE CASCADE ON DELETE CASCADE,
    UNIQUE(book_id, subject_area_id)
);