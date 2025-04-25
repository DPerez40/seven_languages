use std::collections::HashMap;

fn main() {
    let mut library = Library::new();

    library.add_book(Book {
        title: String::from("Dune"),
        author: String::from("Frankie Herbert"),
        year: 1965,
        borrowed: false,
    });

    library.add_book(Book {
        title: String::from("Musashi"),
        author: String::from("Eiji Yoshikawa"),
        year: 1935,
        borrowed: false,
    });

    library.list_books();

    library.remove_book("The Hobbit");
    library.remove_book("Dune");

    library.borrow_book("Musashi");
    library.list_books();
    library.return_book("Musashi");
    library.list_books();

    library.search_books("Dune");
    library.search_books("Eiji");
    library.search_books("Star Wars");
}

struct Book {
    title: String,
    author: String,
    year: u32,
    borrowed: bool,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn search_books(&self, query: &str) {
        let mut found = false;
        for book in &self.books {
            if book.title.to_lowercase().contains(&query.to_lowercase())
                || book.author.to_lowercase().contains(&query.to_lowercase())
            {
                println!("- {} by {} ({})", book.title, book.author, book.year);
                found = true;
            }
        }

        if !found {
            println!("No books found matching '{}'", query);
        }
    }

    fn borrow_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                if book.borrowed {
                    println!("'{}' is already borrowed.", book.title);
                } else {
                    book.borrowed = true;
                    println!("You borrowed '{}'.", book.title);
                }
                return;
            }
        }
        println!("Book '{}' not found.", title);
    }

    fn return_book(&mut self, title: &str) {
        for book in &mut self.books {
            if book.title == title {
                if !book.borrowed {
                    println!("'{}' is already returned.", book.title);
                } else { 
                    book.borrowed = false;
                    println!("You returned '{}'.", book.title);
                }
                return;
            }
        }
        println!("Book '{}' not found.", title);
    }

    fn add_book(&mut self, book: Book) {
        println!("Adding book: {}", book.title);
        self.books.push(book);
    }

    fn remove_book(&mut self, title: &str) {
        match self.books.iter().position(|b| b.title == title) {
            Some(index) => {
                self.books.remove(index);
                println!("Removed book: {}", title);
            }
            None => println!("Book '{}' not found.", title),
        }
    }

    fn list_books(&self) {
        if self.books.is_empty() {
            println!("Library is empty.");
        } else {
            println!("Books in the library:");
            for book in &self.books {
                println!("- {} by {}, published in year {}, status: {}", book.title, book.author, book.year,
            if book.borrowed {"Borrowed"} else {"Available"});
            }
        }
    }
}

