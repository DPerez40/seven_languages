// Borrowable Trait
trait Borrowable {
    fn borrow_item(&mut self);
    fn return_item(&mut self);
    fn status(&self) -> String;
}

// Book & Magazine Structs
struct Book {
    title: String,
    author: String,
    year: u32,
    borrowed: bool,
}

struct Magazine {
    title: String,
    issue: String,
    borrowed: bool,
}

// Implement Traits
impl Borrowable for Book {
    fn borrow_item(&mut self) {
        if !self.borrowed {
            self.borrowed = true;
            println!("Borrowed book: {}", self.title);
        } else {
            println!("Book {} is already borrowed.", self.title);
        }
    }

    fn return_item(&mut self) {
        if self.borrowed {
            self.borrowed = false;
            println!("Returned book: {}", self.title);
        } else {
            println!("The book '{}' is not borrowed yet.  Cannot return something not borrowed.", self.title);
        }
    }

    fn status(&self) -> String {
        if self.borrowed {
            format!("{} (Borrowed)", self.title)
        } else {
            format!("{} (Available)", self.title)
        }
    }
}

impl Borrowable for Magazine {
    fn borrow_item(&mut self) {
        if !self.borrowed {
            self.borrowed = true;
            println!("Borrowed Magazine: {}", self.title);
        } else {
            println!("'{}' is already borrowed.", self.title);
        }
    }

    fn return_item(&mut self) {
        if self.borrowed {
            self.borrowed = false;
            println!("The magazine '{}' has been returned.", self.title);
        } else {
            println!("'{}' is not borrowed and can therefore not be returned.", self.title);
        }
    }

    fn status(&self) -> String {
        if self.borrowed {
            format!("'{}' (Borrowed)", self.title)
        } else {
            format!("'{}' (Available)", self.title)
        }
    }
}

// Create Library to store

struct Library {
    items: Vec<Box<dyn Borrowable>>,
}

impl Library {
    fn new() -> Self {
        Library { items: Vec::new() }
    }

    fn add_item(&mut self, item: Box<dyn Borrowable>) {
        self.items.push(item);
    }

    fn list_items(&self) {
        println!("Items in library:");
        for item in &self.items {
            println!("- {}", item.status());
        }
    }

    fn borrow_item(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            item.borrow_item();
        } else {
            println!("Invalid item index.");
        }
    }

    fn return_item(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            item.return_item();
        } else {
            println!("Invalid item index.");
        }
    }

    fn search(&self, keyword: &str) {
        println!("Search results for {}", keyword);
        let mut found = false;

        for (i, item) in self.items.iter().enumerate() {
            let status = item.status();
            if status.to_lowercase().contains(&keyword.to_lowercase()) {
                println!("{}: {}", i, status);
                found = true;
            } 
        }

        if !found {
            println!("No items found for '{}'.", keyword);
        }
    }
}

// Test in main()

fn main() {
    let mut library = Library::new();

    let book1 = Book {
        title: String::from("Dune"),
        author: String::from("Frank Herbert"),
        year: 1965,
        borrowed: false,
    };

    let magazine1 = Magazine {
        title: String::from("Scientific American"),
        issue: String::from("March 2024"),
        borrowed: false,
    };

    library.add_item(Box::new(book1));
    library.add_item(Box::new(magazine1));

    library.list_items();

    library.borrow_item(0);
    library.return_item(0);
    library.borrow_item(1);
    library.list_items();

    library.search("dune");
    library.search("scientific");
    library.search("musashi");
}



