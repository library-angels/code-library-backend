ALTER TABLE books DROP COLUMN series_id;

CREATE TABLE books_series (
    id SERIAL PRIMARY KEY,
    book_id INTEGER NOT NULL REFERENCES books ON UPDATE CASCADE ON DELETE CASCADE,
    series_id INTEGER NOT NULL REFERENCES series ON UPDATE CASCADE ON DELETE CASCADE,
    -- `book_id` set `UNIQUE`, cause we assume that one book will only be part of one series
    UNIQUE(book_id)
);
