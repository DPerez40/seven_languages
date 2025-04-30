mod library;

use library::book::Book;
use library::magazine::Magazine;
use library::user::User;
use library::traits::Borrowable;
use library::library::Library; 

fn main() {
    let mut library = Library {
        items: Vec::new(),
        books: Vec::new(),
        magazines: Vec::new(),
    };

    let book1 = Book {
        title: String::from("Dune"),
        author: String::from("Frank Herbert"),
        year: 1965,
        borrowed: false,
        borrowed_date: None,
        due_date: None,
        reservations: Vec::new(),
    };

    let magazine1 = Magazine {
        title: String::from("Scientific American"),
        issue: String::from("March 2024"),
        borrowed: false,
        borrowed_date: None,
        due_date: None,
        reservations: Vec::new(),
    };

    library.books.push(book1.clone());
    library.magazines.push(magazine1.clone());
    library.items.push(Box::new(book1));
    library.items.push(Box::new(magazine1));

    let mut user1 = User::new("Dakota");

    library.borrow_item(0);
    library.reserve_item(0, String::from("Brutus"));
    library.return_item(0, &mut user1);

    library.save_to_file();

    println!("\n--- Restarting and loading saved library ---");
    let mut new_library = Library::new();
    new_library.load_from_file();
    new_library.list_items();
}
