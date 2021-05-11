CREATE TABLE categories (
    id UUID,
    name VARCHAR(100) UNIQUE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE languages (
    id UUID,
    iso_code VARCHAR(3) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE persons (
    id UUID,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    date_of_birth DATE,
    isni VARCHAR(16) UNIQUE,
    orcid VARCHAR(16) UNIQUE,
    oclc INTEGER UNIQUE,
    PRIMARY KEY (id)
);

CREATE TABLE publishers (
    id UUID,
    name VARCHAR(100) UNIQUE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE series (
    id UUID,
    publisher_id UUID NOT NULL,
    name VARCHAR(100) NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (publisher_id) REFERENCES publishers (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(publisher_id, name)
);

CREATE TABLE subject_areas (
    id UUID,
    name VARCHAR(100) UNIQUE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE tags (
    id UUID,
    name VARCHAR(50) UNIQUE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE books (
    id UUID,
    code_identifier INTEGER NOT NULL CHECK (code_identifier >= 0),
    isbn VARCHAR(13) UNIQUE,
    issn VARCHAR(8) UNIQUE,
    release_year SMALLINT NOT NULL CHECK (release_year >= 0),
    edition INTEGER,
    pages INTEGER CHECK (pages >= 1),
    subtitle VARCHAR(500),
    title VARCHAR(100) NOT NULL,
    description TEXT,
    category_id UUID NOT NULL,
    language_id UUID NOT NULL,
    publisher_id UUID NOT NULL,
    series_id UUID,
    PRIMARY KEY (id),
    FOREIGN KEY (category_id) REFERENCES categories (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (language_id) REFERENCES languages (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (publisher_id) REFERENCES publishers (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (series_id) REFERENCES series (id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TABLE books_authors (
    id UUID,
    book_id UUID NOT NULL,
    person_id UUID NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (book_id) REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (person_id) REFERENCES persons (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(book_id, person_id)
);

CREATE TABLE books_editors (
    id UUID,
    book_id UUID NOT NULL,
    person_id UUID NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (book_id) REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    FOREIGN KEY (person_id) REFERENCES persons (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(book_id, person_id)
);

CREATE TABLE books_subject_areas (
    id UUID,
    book_id UUID NOT NULL,
    subject_area_id UUID NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (book_id) REFERENCES books (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (subject_area_id) REFERENCES subject_areas (id) ON UPDATE CASCADE ON DELETE CASCADE,
    UNIQUE(book_id, subject_area_id)
);

CREATE TABLE books_tags (
    id UUID,
    book_id UUID NOT NULL,
    tag_id UUID NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (book_id) REFERENCES books (id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON UPDATE CASCADE ON DELETE CASCADE,
    UNIQUE(book_id, tag_id)
);

CREATE TABLE copies (
    id UUID,
    book_id UUID NOT NULL,
    code_identifier_copy_id INTEGER NOT NULL CHECK (code_identifier_copy_id >= 0),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_by UUID NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (book_id) REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(book_id,code_identifier_copy_id)
);
