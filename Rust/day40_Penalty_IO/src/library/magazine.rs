use std::time::{SystemTime, Duration};
use serde::{Serialize, Deserialize};
use crate::library::traits::Borrowable;
use crate::library::user::User;

#[derive(Serialize, Deserialize, Clone)]
pub struct Magazine {
    pub title: String,
    pub issue: String,
    pub borrowed: bool,
    pub borrowed_date: Option<SystemTime>,
    pub due_date: Option<SystemTime>,
    pub reservations: Vec<String>,
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

    fn as_any(&mut self) -> &mut dyn std::any::Any {
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
