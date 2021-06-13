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
    copy_id INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_by UUID NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (book_id) REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
    UNIQUE(book_id,copy_id)
);

CREATE FUNCTION copies_copy_id_create_seq() RETURNS trigger AS
  $$
  BEGIN
    IF EXISTS (SELECT FROM copies WHERE book_id = NEW.book_id) THEN
      NEW.copy_id := nextval(format('copies_copy_id_%s_seq', NEW.book_id));
      return NEW;
    ELSE
      EXECUTE format('CREATE SEQUENCE "copies_copy_id_%s_seq" MINVALUE 0', NEW.book_id);
      NEW.copy_id := nextval(format('copies_copy_id_%s_seq', NEW.book_id));
      return NEW;
    END IF;
  END
  $$ LANGUAGE plpgsql;

CREATE TRIGGER copies_copy_id_create_seq BEFORE INSERT ON copies FOR EACH ROW EXECUTE PROCEDURE copies_copy_id_create_seq();

CREATE FUNCTION copies_copy_id_delete_seq() RETURNS trigger AS
  $$
  BEGIN
    IF NOT EXISTS (SELECT FROM copies WHERE book_id = OLD.book_id) THEN
      EXECUTE format('DROP SEQUENCE "copies_copy_id_%s_seq"', OLD.book_id);
    END IF;
    RETURN OLD;
  END
  $$ LANGUAGE plpgsql;

CREATE TRIGGER copies_copy_id_delete_seq AFTER DELETE ON copies FOR EACH ROW EXECUTE PROCEDURE copies_copy_id_delete_seq();
