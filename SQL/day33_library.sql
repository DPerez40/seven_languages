-- Schema:

CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  title TEXT NOT NULL,
  author TEXT NOT NULL,
  year_published INT NOT NULL,
  available BOOLEAN DEFAULT TRUE
);

CREATE TABLE borrowers (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE book_log (
  id SERIAL PRIMARY KEY,
  book_id INTEGER REFERENCES books(id),
  borrower_id INTEGER REFERENCES borrowers(id),
  action TEXT CHECK (action IN ('borrowed', 'returned')),
  timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Query:

-- Add borrower_id
ALTER TABLE books
ADD COLUMN borrower_id INTEGER REFERENCES borrowers(id);


-- Insert books
INSERT INTO books (title, author, year_published, available) VALUES
('Dune', 'Frank Herbert', 1965, TRUE),
('Musashi', 'Eiji Yoshikawa', 1935, TRUE),
('1984', 'George Orwell', 1949, TRUE),
('Snow Crash', 'Neal Stephenson', 1992, TRUE),
('The Hobbit', 'J.R.R. Tolkien', 1937, TRUE);

-- Insert borrowers
INSERT INTO borrowers (name) VALUES
('Francisco Perez'),
('Alex Fisher');

-- Simulate a checkout
UPDATE books
SET available = FALSE,
    borrower_id = (SELECT id FROM borrowers WHERE name = 'Francisco Perez')
WHERE title = '1984' AND available = TRUE;

-- Log the checkout
INSERT INTO book_log (book_id, borrower_id, action)
SELECT id, borrower_id, 'borrowed'
FROM books
WHERE title = '1984';

-- Simulate a return
UPDATE books
SET available = TRUE,
    borrower_id = NULL
WHERE title = '1984' AND available = FALSE;

-- Log the return
INSERT INTO book_log (book_id, borrower_id, action)
SELECT id, (SELECT id FROM borrowers WHERE name = 'Dakota Perez'), 'returned'
FROM books
WHERE title = '1984';

-- Show book status with borrower
SELECT
  books.title,
  books.author,
  borrowers.name AS borrowed_by,
  CASE
    WHEN books.available = FALSE AND borrowers.name IS NOT NULL THEN 'Checked Out'
    WHEN books.available = TRUE AND borrowers.name IS NULL THEN 'Available'
    ELSE 'Unknown'
  END AS status_label
FROM books
LEFT JOIN borrowers ON books.borrower_id = borrowers.id
ORDER BY books.id;

-- Show activity log
SELECT
  book_log.id,
  books.title,
  borrowers.name,
  book_log.action,
  book_log.timestamp
FROM book_log
JOIN books ON book_log.book_id = books.id
JOIN borrowers ON book_log.borrower_id = borrowers.id
ORDER BY book_log.timestamp;


CREATE VIEW library_summary AS
SELECT 
	books.title,
    books.author,
    books.available,
    log_latest.action AS last_action,
    borrowers.name AS last_borrower,
    log_latest.timestamp
    
FROM books

LEFT JOIN (
  SELECT DISTINCT ON (book_id)
  	book_id,
    action,
    borrower_id,
  	timestamp
  FROM book_log
  ORDER BY book_id, timestamp DESC
) AS log_latest ON books.id = log_latest.book_id

LEFT JOIN borrowers ON log_latest.borrower_id = borrowers.id;

SELECT * FROM library_summary;
