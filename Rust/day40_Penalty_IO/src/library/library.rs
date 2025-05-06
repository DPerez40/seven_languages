use crate::library::book::Book;
use crate::library::magazine::Magazine;
use crate::library::traits::Borrowable;
use crate::library::user::User;
use std::fs::File;
use std::io::Write;
use std::fs;
use serde_json;

pub struct Library {
    pub items: Vec<Box<dyn Borrowable>>,
    pub books: Vec<Book>,
    pub magazines: Vec<Magazine>,
    pub users: Vec<User>,
}

impl Library {
    pub fn new() -> Self {
        Library {
            items: Vec::new(),
            books: Vec::new(),
            magazines: Vec::new(),
            users: Vec::new(),
        }
    }

    pub fn list_users(&self) {
        println!("\n----- Library Users -----");
        for user in &self.users {
            println!("{} owes ${}", user.name, user.penalty_fees);
        }
    }

    pub fn add_user(&mut self, name: &str) {
        if self.users.iter().any(|u| u.name == name) {
            println!("User '{}' already exists.", name);
        } else {
            let user = User::new(name);
            self.users.push(user);
            println!("User '{}' has been added to the system.", name);
        }
    }

    pub fn get_user_mut(&mut self, name: &str) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.name == name)
    }

    pub fn add_item(&mut self, item: Box<dyn Borrowable>) {
        self.items.push(item);
    }

    pub fn reserve_item(&mut self, index: usize, user_name: String) {
        if let Some(item) = self.items.get_mut(index) {
            item.reserve_item(user_name);
        } else {
            println!("Invalid item index.");
        }
    }

    pub fn list_items(&self) {
        println!("\n----- Items in library: -----");
        for (i, item) in self.items.iter().enumerate() {
            println!("{}. {}", i, item.status());
        }
    }

    pub fn borrow_item(&mut self, index: usize) {
        if let Some(item) = self.items.get_mut(index) {
            if item.is_borrowed() {
                println!("That item is already borrowed.");
            } else {
                item.borrow_item();
                println!("Item borrowed successfully.");
            }
        } else {
            println!("Invalid item index.");
        }
    }

    pub fn return_item(&mut self, index: usize, user: &mut User) {
        if let Some(item) = self.items.get_mut(index) {
            item.return_item(user);
        } else {
            println!("Invalid item index.");
        }
    }

    pub fn search(&self, keyword: &str) {
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

    pub fn save_to_file(&mut self) {
        self.books.clear();
        self.magazines.clear();
        for item in &mut self.items{
            if let Some(book) = item.as_any().downcast_ref::<Book>() {
                self.books.push(book.clone());
            } else if let Some(magazine) = item.as_any().downcast_ref::<Magazine>() {
                self.magazines.push(magazine.clone());
            }
        }
        let books_json = serde_json::to_string_pretty(&self.books).expect("Failed to serialize books.");
        let magazines_json = serde_json::to_string_pretty(&self.magazines).expect("Failed to serialize magazines.");
        let users_json = serde_json::to_string_pretty(&self.users).unwrap();

        let mut file = File::create("library_save.json").expect("Failed to create new file.");

        writeln!(file, "{{").unwrap();
        writeln!(file, "\"books\": {},", books_json).unwrap();
        writeln!(file, "\"magazines\": {},", magazines_json).unwrap();
        writeln!(file, "\"users\": {}", users_json).unwrap();
        writeln!(file, "}}").unwrap();

        println!("Library saved to 'library_save.json'");
    }

    pub fn load_from_file(&mut self) {
        let data = fs::read_to_string("library_save.json").expect("Failed to read file");

        let parsed: serde_json::Value = serde_json::from_str(&data).expect("Failed to parse JSON.");

        let books: Vec<Book> = serde_json::from_value(parsed["books"].clone()).expect("Failed to deserialize books.");
        let magazines: Vec<Magazine> = serde_json::from_value(parsed["magazines"].clone()).expect("Failed to deserialize magazines.");
        let users: Vec<User> = serde_json::from_value(parsed["users"].clone()).expect("Failed to deserialize users.");

        self.books = books;
        self.magazines = magazines;
        self.users = users;

        self.items.clear();
        for book in self.books.iter() {
            self.items.push(Box::new(book.clone()) as Box<dyn Borrowable>);
        }
        for mag in self.magazines.iter() {
            self.items.push(Box::new(mag.clone()) as Box<dyn Borrowable>);
        }

        println!("Library loaded from 'library_save.json'.");
    }
}
