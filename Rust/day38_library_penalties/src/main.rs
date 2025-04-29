use std::time::{SystemTime, Duration};
use std::any::Any;

// Borrowable Trait
trait Borrowable: Any {
    fn borrow_item(&mut self);
    fn return_item(&mut self);
    fn status(&self) -> String;
    fn as_any(&mut self) -> &mut dyn Any;
}

// Book and Magazine Structs
struct Book {
    title: String,
    author: String,
    year: u32,
    borrowed: bool,
    borrowed_date: Option<SystemTime>,
    due_date: Option<SystemTime>,
}

struct Magazine {
    title: String,
    issue: String,
    borrowed: bool,
    borrowed_date: Option<SystemTime>,
    due_date: Option<SystemTime>,
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

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn return_item(&mut self) {
        if self.borrowed {
            let now = SystemTime::now();
            if let Some(due) = self.due_date {
                if now > due {
                    let overdue_secs = now.duration_since(due).unwrap().as_secs();
                    let overdue_days = overdue_secs / (24 * 60 * 60);
                    let penalty = overdue_days *  1; // $1 per day overdue
                    println!("Returned book '{}' late by {} days. Penalty: ${}", self.title, overdue_days, penalty);
                } else {
                    println!("Returned '{}' on time!", self.title);
                }
            }
            self.borrowed = false;
            self.borrowed_date = None;
            self.due_date = None;
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

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn return_item(&mut self) {
        if self.borrowed {
            let now = SystemTime::now();
            if let Some(due) = self.due_date {
                if now > due {
                    let overdue_secs = now.duration_since(due).unwrap().as_secs();
                    let overdue_days = overdue_secs / (24 * 60 * 60);
                    let penalty = overdue_days * 1;
                    println!("Returned magazine '{}' late by {} days.  Penalty: ${}", self.title, overdue_days, penalty);
                } else {
                    println!("Returned magazine '{}' on time!", self.title);
                }
            }
            self.borrowed = false;
            self.borrowed_date = None;
            self.due_date = None;
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

    fn return_item(&mut self, index: usize, user: &mut User) {
        if let Some(item) = self.items.get_mut(index) {
            if let Some(book) = item.as_any().downcast_mut::<Book>() {
                if book.borrowed {
                    let now = SystemTime::now();
                    if let Some(due) = book.due_date {
                        if now > due {
                            let overdue_secs = now.duration_since(due).unwrap().as_secs();
                            let overdue_days = overdue_secs / (24 * 60 * 60);
                            let penalty = overdue_days as u32 * 1;
                            println!("Returned book '{}' late by {} days. Penalty: ${}", book.title, overdue_days, penalty);
                            user.add_penalty(penalty);
                        } else {
                            println!("Returned '{}' on time!", book.title);
                        }
                    }
                    book.borrowed = false;
                    book.borrowed_date = None;
                    book.due_date = None;
                } else {
                    println!("The book '{}' is not borrowed yet. Cannot return.", book.title);
                }
                return;
            }

            if let Some(magazine) = item.as_any().downcast_mut::<Magazine>() {
                if magazine.borrowed {
                    let now = SystemTime::now();
                    if let Some(due) = magazine.due_date {
                        if now > due {
                            let overdue_secs = now.duration_since(due).unwrap().as_secs();
                            let overdue_days = overdue_secs / (24 * 60 * 60);
                            let penalty = overdue_days as u32 * 1;
                            println!("Returned magazine '{}' late by {} days. Penalty: ${}", magazine.title, overdue_days, penalty);
                            user.add_penalty(penalty);
                        } else {
                            println!("Returned magazine '{}' on time!", magazine.title);
                        }
                    }
                    magazine.borrowed = false;
                    magazine.borrowed_date = None;
                    magazine.due_date = None;
                } else {
                    println!("The magazine '{}' is not borrowed yet and cannot return until borrowed.  This is the way.", magazine.title);
                }
            }
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
        borrowed_date: None,
        due_date: None,
    };

    let magazine1 = Magazine {
        title: String::from("Scientific American"),
        issue: String::from("March 2024"),
        borrowed: false,
        borrowed_date: None,
        due_date: None,
    };

    library.add_item(Box::new(book1));
    library.add_item(Box::new(magazine1));

    let mut user1 = User::new("Dakota");
    let mut user2 = User::new("Brutus");

    library.list_items();

    println!("\nBorrowing Book 0:");
    library.borrow_item(0);

    println!("\nReturning Book 0:");
    library.return_item(0, &mut user1);

    println!("Borrowing Magazine 1:");
    library.borrow_item(1);

    println!("Returning Magazine 1:");
    library.return_item(1, &mut user2);

    println!("\nFinal penalty balance for  {}: ${}", user1.name, user1.penalty_fees);

    library.search("dune");
    library.search("scientific");
    library.search("musashi");

    println!("\n\n{} decides to pay $5 toward their penalty:", user1.name);
    user1.pay_penalty(5);
    
}



