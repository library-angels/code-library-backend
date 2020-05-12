--
-- TESTING DATABASE
--

---------------------------------- SCHEMA----------------------------------

CREATE DATABASE test_db;
\c test_db

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  sub VARCHAR(255) NOT NULL,
  email VARCHAR(50) NOT NULL,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  picture bytea,
  oauth_access_token VARCHAR(256) NOT NULL,
  oauth_access_token_valid TIMESTAMP NOT NULL,
  oauth_refresh_token VARCHAR(256) NOT NULL,
  oauth_refresh_token_valid TIMESTAMP NOT NULL
);

CREATE TABLE roles (
  id SERIAL PRIMARY KEY,
  role_name VARCHAR(50) NOT NULL,
  access_manage_books BOOLEAN NOT NULL,
  access_manage_roles BOOLEAN NOT NULL
);

CREATE TABLE users_roles (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE UNIQUE,
  role_id INTEGER NOT NULL REFERENCES roles (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL
);

CREATE TABLE designations (
  id SERIAL PRIMARY KEY,
  name VARCHAR(10) NOT NULL
);

CREATE TABLE persons (
  id SERIAL PRIMARY KEY,
  first_name VARCHAR(50),
  last_name VARCHAR(50),
  date_of_birth DATE,
  isni VARCHAR(20),
  orcid VARCHAR(20),
  oclc VARCHAR(20)
);

CREATE TABLE publishers (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL
);

CREATE TABLE series (
  id SERIAL PRIMARY KEY,
  publisher_id INTEGER NOT NULL REFERENCES publishers (id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TABLE languages (
  id SERIAL PRIMARY KEY,
  iso_code VARCHAR(7) NOT NULL,
  language_name VARCHAR(200) NOT NULL
);

CREATE TABLE physical_sizes (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL
);

CREATE TABLE subject_areas (
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL
);

CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  isbn_13 VARCHAR(13),
  issn VARCHAR(20),
  title VARCHAR(200) NOT NULL,
  subtitle VARCHAR(200),
  description TEXT,
  cover INTEGER,
  edition INTEGER,
  release_year SMALLINT CHECK (release_year >= 0),
  pages INTEGER,
  code_identifier VARCHAR(10) UNIQUE,
  publisher_id INTEGER NOT NULL REFERENCES publishers (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  designation_id INTEGER NOT NULL REFERENCES designations (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  series_id INTEGER REFERENCES series (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  language_id INTEGER NOT NULL REFERENCES languages (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  physical_size_id INTEGER NOT NULL REFERENCES physical_sizes (id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TABLE books_subject_areas (
  id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE CASCADE,
  subject_area_id INTEGER NOT NULL REFERENCES subject_areas (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE books_tags (
  id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE CASCADE,
  tag_id INTEGER NOT NULL REFERENCES tags (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE statuses (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL UNIQUE,
  available BOOLEAN NOT NULL,
  bookable BOOLEAN NOT NULL
);

CREATE TABLE copies (
  id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  status_id INTEGER NOT NULL REFERENCES statuses (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  code_identifier_copy_id INTEGER,
  date_added DATE NOT NULL DEFAULT CURRENT_DATE
);

CREATE TABLE books_authors (
  id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  person_id INTEGER NOT NULL REFERENCES persons (id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE TABLE books_editors (
  id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL REFERENCES books (id) ON UPDATE CASCADE ON DELETE RESTRICT,
  person_id INTEGER NOT NULL REFERENCES persons (id) ON UPDATE CASCADE ON DELETE RESTRICT
);

CREATE FUNCTION copies_book_id_create_seq() RETURNS trigger AS
  $$
  BEGIN
    IF EXISTS (SELECT * FROM copies WHERE book_id = NEW.book_id) THEN
      NEW.code_identifier_copy_id := nextval(format('copies_code_identifier_%s_seq', NEW.book_id));
      return NEW;
    ELSE
      EXECUTE format('CREATE SEQUENCE copies_code_identifier_%s_seq', NEW.book_id);
      NEW.code_identifier_copy_id := nextval(format('copies_code_identifier_%s_seq', NEW.book_id));
      return NEW;
    END IF;
  END
  $$ LANGUAGE plpgsql;

CREATE TRIGGER copies_book_id_create_seq BEFORE INSERT ON copies FOR EACH ROW EXECUTE PROCEDURE copies_book_id_create_seq();

CREATE FUNCTION copies_book_id_delete_seq() RETURNS trigger AS
  $$
  BEGIN
    IF (SELECT COUNT(*) FROM copies WHERE book_id = OLD.book_id) = 0 THEN
      EXECUTE format('DROP SEQUENCE copies_code_identifier_%s_seq', OLD.book_id);
    END IF;
    RETURN OLD;
  END
  $$ LANGUAGE plpgsql;

CREATE TRIGGER copies_book_id_delete_seq AFTER DELETE ON copies FOR EACH ROW EXECUTE PROCEDURE copies_book_id_delete_seq();


CREATE TABLE copies_users_active (
  id SERIAL PRIMARY KEY,
  copy_id INTEGER NOT NULL REFERENCES copies (id) ON UPDATE CASCADE ON DELETE RESTRICT UNIQUE, -- do not allow to delete a book copy, if it is lend to a user
  user_id INTEGER NOT NULL REFERENCES users (id) ON UPDATE CASCADE ON DELETE RESTRICT, -- do not allow to delete a user account, if it has active borrows
  borrow_from DATE NOT NULL,
  borrow_to DATE NOT NULL
);

CREATE TABLE copies_users_history (
  id SERIAL PRIMARY KEY,
  copy_id INTEGER NOT NULL REFERENCES copies (id) ON UPDATE CASCADE ON DELETE CASCADE,
  user_id INTEGER NOT NULL REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
  borrow_start DATE NOT NULL,
  borrow_end DATE NOT NULL
);

CREATE TABLE copies_users_reserved (
  id SERIAL PRIMARY KEY,
  copy_id INTEGER NOT NULL REFERENCES copies (id) ON UPDATE CASCADE ON DELETE CASCADE,
  user_id INTEGER NOT NULL REFERENCES users (id) ON UPDATE CASCADE ON DELETE CASCADE,
  duration INTERVAL NOT NULL
);

----------------------------------BASE DATASET----------------------------------

INSERT INTO designations (name) VALUES ('STS');
INSERT INTO designations (name) VALUES ('SE');
INSERT INTO designations (name) VALUES ('ID');
INSERT INTO designations (name) VALUES ('PM');
INSERT INTO designations (name) VALUES ('IS');
INSERT INTO designations (name) VALUES ('DH');

INSERT INTO languages (iso_code, language_name) VALUES ('eng','English');
INSERT INTO languages (iso_code, language_name) VALUES ('rus','Russian');
INSERT INTO languages (iso_code, language_name) VALUES ('ger','German');

INSERT INTO physical_sizes (name) VALUES ('normal');
INSERT INTO physical_sizes (name) VALUES ('xxl');

INSERT INTO roles (role_name, access_manage_books, access_manage_roles) VALUES ('User', False, False);
INSERT INTO roles (role_name, access_manage_books, access_manage_roles) VALUES ('Manager', True, False);
INSERT INTO roles (role_name, access_manage_books, access_manage_roles) VALUES ('Administrator', True, True);

INSERT INTO statuses (name, available, bookable) VALUES ('available', True, True);
INSERT INTO statuses (name, available, bookable) VALUES ('new', False, False);
INSERT INTO statuses (name, available, bookable) VALUES ('presence', True, False);
INSERT INTO statuses (name, available, bookable) VALUES ('missing', False, False);

----------------------------------TEST DATA---------------------------------------

INSERT INTO publishers (name)
	SELECT 'Vintage Books' WHERE NOT EXISTS
		(SELECT * FROM publishers WHERE name = 'Vintage Books');

INSERT INTO subject_areas (name)
	SELECT 'Health Care' WHERE NOT EXISTS
		(SELECT * FROM subject_areas WHERE name = 'Health Care');

INSERT INTO persons (first_name, last_name)
	SELECT 'David', 'Goldhill' WHERE NOT EXISTS
		(SELECT * FROM persons WHERE first_name = 'David'
			AND last_name = 'Goldhill');

INSERT INTO books (isbn_13, issn, title, subtitle, description, cover, edition, release_year, pages,
	code_identifier, publisher_id, designation_id, series_id, language_id, physical_size_id)
		VALUES ('9780345802736', NULL,
			'Catastrophic Care', 'Why everything we think we know about health care is wrong',
				NULL, NULL, NULL, 2013, NULL, 'DH01',
				(SELECT id FROM publishers WHERE name = 'Vintage Books'),
					(SELECT id FROM designations WHERE name = 'DH'), NULL,
						(SELECT id FROM languages WHERE iso_code = 'eng'),
							(SELECT id FROM physical_sizes WHERE name = 'normal'));

INSERT INTO copies (book_id, status_id) VALUES
	((SELECT id FROM books WHERE title = 'Catastrophic Care'
		AND subtitle = 'Why everything we think we know about health care is wrong'), 1);

INSERT INTO books_authors (book_id, person_id) VALUES
	((SELECT id FROM books WHERE title = 'Catastrophic Care'
		AND subtitle = 'Why everything we think we know about health care is wrong'),
			(SELECT id FROM persons WHERE first_name = 'David'
				AND last_name = 'Goldhill'));

INSERT INTO books_subject_areas (book_id, subject_area_id) VALUES
	((SELECT id FROM books WHERE title = 'Catastrophic Care'
		AND subtitle = 'Why everything we think we know about health care is wrong'),
			(SELECT id FROM subject_areas WHERE name = 'Health Care'));

----------------------------------------------------------------------------------

INSERT INTO publishers (name)
	SELECT 'CRC Press' WHERE NOT EXISTS
		(SELECT * FROM publishers WHERE name = 'CRC Press');

INSERT INTO subject_areas (name)
	SELECT 'Computer Graphics' WHERE NOT EXISTS
		(SELECT * FROM subject_areas WHERE name = 'Computer Graphics');

INSERT INTO persons (first_name, last_name)
	SELECT 'Sumanta', 'Guha' WHERE NOT EXISTS
		(SELECT * FROM persons WHERE first_name = 'Sumanta'
			AND last_name = 'Guha');

INSERT INTO books (isbn_13, issn, title, subtitle, description, cover, edition, release_year, pages,
	code_identifier, publisher_id, designation_id, series_id, language_id, physical_size_id)
	VALUES ('9781138612648', NULL, 'Computer Graphics Through Opengl From Theory to Experiments',
		'Comprehensive coverage of shaders and the programmable pipline',
		NULL, NULL, NULL, 2019, NULL, 'XXL12',
			(SELECT id FROM publishers WHERE name = 'CRC Press'),
				(SELECT id FROM designations WHERE name = 'SE'), NULL,
					(SELECT id FROM languages WHERE iso_code = 'eng'),
						(SELECT id FROM physical_sizes WHERE name = 'xxl'));

INSERT INTO copies (book_id, status_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Graphics Through Opengl From Theory to Experiments'
			AND subtitle = 'Comprehensive coverage of shaders and the programmable pipline'), 1);

INSERT INTO books_authors (book_id, person_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Graphics Through Opengl From Theory to Experiments'
		AND subtitle = 'Comprehensive coverage of shaders and the programmable pipline'),
			(SELECT id FROM persons WHERE first_name = 'Sumanta'
				AND last_name = 'Guha'));

INSERT INTO books_subject_areas (book_id, subject_area_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Graphics Through Opengl From Theory to Experiments'
			AND subtitle = 'Comprehensive coverage of shaders and the programmable pipline'),
				(SELECT id FROM subject_areas WHERE name = 'Computer Graphics'));

-------------------------------------------------------------------------------------
INSERT INTO publishers (name)
	SELECT 'FREEMAN and Company' WHERE NOT EXISTS
		(SELECT * FROM publishers WHERE name = 'FREEMAN and Company');

INSERT INTO subject_areas (name)
	SELECT 'Philosophy' WHERE NOT EXISTS
		(SELECT * FROM subject_areas WHERE name = 'Philosophy');

INSERT INTO subject_areas (name)
	SELECT 'Computer Science' WHERE NOT EXISTS
		(SELECT * FROM subject_areas WHERE name = 'Computer Science');

INSERT INTO persons (first_name, last_name)
	SELECT 'Joseph', 'Weizenbaum' WHERE NOT EXISTS
		(SELECT * FROM persons WHERE first_name = 'Joseph'
			AND last_name = 'Weizenbaum');

INSERT INTO books (isbn_13, issn, title, subtitle, description, cover, edition, release_year, pages,
	code_identifier, publisher_id, designation_id, series_id, language_id, physical_size_id)
		VALUES	('9780552778077', NULL, 'Computer Power and Human Reason', 'From Judgment to Calculation',
			NULL, NULL, NULL, 1976, NULL, 'STS18',
				(SELECT id FROM publishers WHERE name = 'FREEMAN and Company'),
					(SELECT id FROM designations WHERE name = 'STS'), NULL,
						(SELECT id FROM languages WHERE iso_code = 'eng'),
							(SELECT id FROM physical_sizes WHERE name = 'normal'));

INSERT INTO copies (book_id, status_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Power and Human Reason'
		AND subtitle = 'From Judgment to Calculation'), 1);

INSERT INTO books_authors (book_id, person_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Power and Human Reason'
		AND subtitle = 'From Judgment to Calculation'),
			(SELECT id FROM persons WHERE first_name = 'Joseph'
				AND last_name = 'Weizenbaum'));

INSERT INTO books_subject_areas (book_id, subject_area_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Power and Human Reason'
		AND subtitle = 'From Judgment to Calculation'),
			(SELECT id FROM subject_areas WHERE name = 'Philosophy'));

INSERT INTO books_subject_areas (book_id, subject_area_id) VALUES
	((SELECT id FROM books WHERE title = 'Computer Power and Human Reason'
			AND subtitle = 'From Judgment to Calculation'),
				(SELECT id FROM subject_areas WHERE name = 'Computer Science'));

\c test_db
