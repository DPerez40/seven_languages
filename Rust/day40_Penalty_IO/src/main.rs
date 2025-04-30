use std::time::{SystemTime, Duration};
use std::any::Any;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::fs;

// Borrowable Trait
trait Borrowable: Any {
    fn borrow_item(&mut self);
    fn return_item(&mut self, user: &mut User);
    fn reserve_item(&mut self, user: String);
    fn status(&self) -> String;
    fn as_any(&mut self) -> &mut dyn Any;
}

// Book and Magazine Structs
#[derive(Serialize, Deserialize, Clone)]
struct Book {
    title: String,
    author: String,
    year: u32,
    borrowed: bool,
    borrowed_date: Option<SystemTime>,
    due_date: Option<SystemTime>,
    reservations: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Magazine {
    title: String,
    issue: String,
    borrowed: bool,
    borrowed_date: Option<SystemTime>,
    due_date: Option<SystemTime>,
    reservations: Vec<String>,
}

// Implement Traits
impl Borrowable for Book {
    fn borrow_item(&mut self) {
        if !self.borrowed {
            self.borrowed = true;
            let now = SystemTime::now();
            self.borrowed_date = Some(now);
            self.due_date = Some(now + Duration::new(14 * 24 * 60 * 60, 0));
            println!("Borrowed book: {} (Due in 14 days)", self.title);
        } else {
            println!("Book {} is already borrowed.", self.title);
        }
    }

    fn reserve_item(&mut self, user: String) {
        self.reservations.push(user.clone());
        println!("{} has been added to the reservation list for '{}'.", user, self.title);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn return_item(&mut self, user: &mut User) {
        if self.borrowed {
            let now = SystemTime::now();
            if let Some(due) = self.due_date {
                if now > due {
                    let overdue_secs = now.duration_since(due).unwrap().as_secs();
                    let overdue_days = overdue_secs / (24 * 60 * 60);
                    let penalty = overdue_days *  1; // $1 per day overdue
                    println!("Returned book '{}' late by {} days. Penalty: ${}", self.title, overdue_days, penalty);
                    user.add_penalty(penalty as u32);
                } else {
                    println!("Returned '{}' on time!", self.title);
                }
            }
            self.borrowed = false;
            self.borrowed_date = None;
            self.due_date = None;
            
            if let Some(next_user) = self.reservations.first() {
                println!("'{}' is now reserved and sent to {}!", self.title, next_user);
                self.borrowed = true;
                self.borrowed_date = Some(SystemTime::now());
                self.due_date = Some(SystemTime::now() + Duration::new(14 * 24 * 60 * 60, 0));
                self.reservations.remove(0);
            }
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
            let now = SystemTime::now();
            self.borrowed_date = Some(now);
            self.due_date = Some(now + Duration::new(14 * 24 * 60 * 60, 0));
            println!("Borrowed Magazine: {} (Due in 14 days)", self.title);
        } else {
            println!("'{}' is already borrowed.", self.title);
        }
    }

    fn reserve_item(&mut self, user: String) {
        self.reservations.push(user.clone());
        println!("{} has been added to the reservation list for '{}'.", user, self.title);
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn return_item(&mut self, user: &mut User) {
        if self.borrowed {
            let now = SystemTime::now();
            if let Some(due) = self.due_date {
                if now > due {
                    let overdue_secs = now.duration_since(due).unwrap().as_secs();
                    let overdue_days = overdue_secs / (24 * 60 * 60);
                    let penalty = overdue_days * 1;
                    println!("Returned magazine '{}' late by {} days.  Penalty: ${}", self.title, overdue_days, penalty);
                    user.add_penalty(penalty as u32);
                } else {
                    println!("Returned magazine '{}' on time!", self.title);
                }
            }
            self.borrowed = false;
            self.borrowed_date = None;
            self.due_date = None;
            if let Some(next_user) = self.reservations.first() {
                println!("'{}' is now reserved and sent to {}!", self.title, next_user);
                self.borrowed = true;
                self.borrowed_date = Some(SystemTime::now());
                self.due_date = Some(SystemTime::now() + Duration::new(14 * 24 * 60 * 60, 0));
                self.reservations.remove(0);
            }
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

// Create Library and User to store
#[derive(Serialize, Deserialize, Clone)]
struct User{
    name: String,
    penalty_fees: u32,
}

impl User {
    fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
            penalty_fees: 0,
        }
    }

    fn add_penalty(&mut self, amount: u32) {
        self.penalty_fees += amount;
        println!("{} now owes a total of: ${}", self.name, self.penalty_fees);
    }

    fn pay_penalty(&mut self, amount: u32) {
        if amount >= self.penalty_fees {
            println!("{} paid off all penalties: ${}", self.name, self.penalty_fees);
            self.penalty_fees = 0;
        } else {
            self.penalty_fees -= amount;
            println!("{} paid ${}. Remaining balance: ${}", self.name, amount, self.penalty_fees);
        }
    }
}

struct Library {
    items: Vec<Box<dyn Borrowable>>,
    books: Vec<Book>,
    magazines: Vec<Magazine>,
}

impl Library {
    fn new() -> Self {
        Library { 
            items: Vec::new(),
            books: Vec::new(),
            magazines: Vec::new(),
        }
    }

    fn add_item(&mut self, item: Box<dyn Borrowable>) {
        self.items.push(item);
    }

    fn reserve_item(&mut self, index: usize, user_name: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.reserve_item(user_name);
        } else {
            println!("Invalid item index.");
        }
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

    fn return_item(&mut self, index: usize, user: &mut User) {
        if let Some(item) = self.items.get_mut(index) {
            item.return_item(user);
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

    fn save_to_file(&self) {
        let books_json = serde_json::to_string_pretty(&self.books).expect("Failed to serialize books.");
        let magazines_json = serde_json::to_string_pretty(&self.magazines).expect("Failed to serialize magazines.");

        let mut file = File::create("library_save.json").expect("Failed to create new file.");

        writeln!(file, "{{").unwrap();
        writeln!(file, "\"books\": {},", books_json).unwrap();
        writeln!(file, "\"magazines\": {}", magazines_json).unwrap();
        writeln!(file, "}}").unwrap();

        println!("Library saved to 'library_save.json'");
    }

    fn load_from_file(&mut self) {
        let data = fs::read_to_string("library_save.json").expect("Failed to read file");

        let parsed: serde_json::Value = serde_json::from_str(&data).expect("Failed to parse JSON.");

        let books: Vec<Book> = serde_json::from_value(parsed["books"].clone()).expect("Failed to deserialize books.");

        let magazines: Vec<Magazine> = serde_json::from_value(parsed["magazines"].clone()).expect("Failed to deeserialize magazines.");

        self.books = books;
        self.magazines = magazines;

        self.items.clear();
        for book in &self.books {
            self.items.push(Box::new(book.clone()));
        }
        for mag in &self.magazines {
            self.items.push(Box::new(mag.clone()));
        }

        println!("Library loaded from 'library_save.json'.");
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

    println!("\n--- Restarting and loading saved library --");
    let mut new_library = Library::new();
    new_library.load_from_file();
    new_library.list_items();
}

