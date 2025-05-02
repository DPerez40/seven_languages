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
        users: Vec::new(),
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

    use std::io::{self, Write};

    loop {
        println!("\n----- Library Menu -----");
        println!("1. List all items");
        println!("2. Borrow Item");
        println!("3. Return Item");
        println!("4. Reserve Item");
        println!("5. List Users");
        println!("6. Add a Book");
        println!("7. Add a Magazine");
        println!("8. Add a User");
        println!("9. Save and Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line.");

        match choice.trim() {
            "1" => library.list_items(),

            "2" => {
                println!("Enter user name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                println!("Enter the item index you want to borrow: ");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    library.borrow_item(index);
                }
            }

            "3" => {
                println!("Enter a user's name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim().to_string();

                println!("Enter the item index for returning item: ");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();

                if let Ok(index) = index_str.trim().parse::<usize>() {
                
                    if let Some(user_index) = library.users.iter().position(|u| u.name == name) {
                        let mut user = library.users.remove(user_index);

                        library.return_item(index, &mut user);

                        library.users.insert(user_index,user);
                    } else {
                        println! ("User '{}' not found.", name);
                    }
                }
            }

            "4" => {
                println!("Enter a user's name: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                println!("What is the item index of the item you are trying to reserve?");
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str).unwrap();
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    library.reserve_item(index, name.to_string());
                }
            }

            "5" => library.list_users(),

            "6" => {
                println!("Enter the book title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                println!("Enter author:");
                let mut author = String::new();
                io::stdin().read_line(&mut author).unwrap();
                let author = author.trim().to_string();

                println!("Enter the year of publication:");
                let mut year_str = String::new();
                io::stdin().read_line(&mut year_str).unwrap();
                let year: u32 = year_str.trim().parse().unwrap_or(0);

                let book = Book {
                    title: title.clone(),
                    author,
                    year,
                    borrowed: false,
                    borrowed_date: None,
                    due_date: None,
                    reservations: Vec::new(),
                };

                library.books.push(book.clone());
                library.items.push(Box::new(book));
                println!("'{}' has been added to the library. Feed me more books!", title);
            }

            "7" => {
                println!("Enter magazine title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_string();

                println!("Enter the issue:");
                let mut issue = String::new();
                io::stdin().read_line(&mut issue).unwrap();
                let issue = issue.trim().to_string();

                let mag = Magazine {
                    title: title.clone(),
                    issue,
                    borrowed: false,
                    borrowed_date: None,
                    due_date: None,
                    reservations: Vec::new(),
                };

                library.magazines.push(mag.clone());
                library.items.push(Box::new(mag));
                println!("Magazine '{}' has been added to the library!", title);
            }

            "8" => {
                println!("Enter the user's name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let name = name.trim();

                library.add_user(name);
                println!("'{}' has been added as a user to the library.", name);
            }

            "9" => {
                library.save_to_file();
                println!("C ya!");
                break;
            }
            _ => println!("You didn't select one of the choices I gave."),
        }

    }

    println!("\n--- Restarting and loading saved library ---");
    let mut new_library = Library::new();
    new_library.load_from_file();
    new_library.list_items();
    
}

