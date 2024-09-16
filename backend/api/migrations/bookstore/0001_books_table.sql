-- Create the book table
CREATE TABLE book (
    id SERIAL PRIMARY KEY,       -- Auto-incrementing ID
    title VARCHAR(255),          -- Title of the book
    author VARCHAR(255),         -- Author of the book
    isbn VARCHAR(13) NOT NULL    -- ISBN of the book (assuming ISBN-13 format)
);

-- Create a unique index on the isbn column
CREATE UNIQUE INDEX idx_book_isbn ON book(isbn);