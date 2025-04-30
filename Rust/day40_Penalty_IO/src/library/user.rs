use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub penalty_fees: u32,
}

impl User {
    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
            penalty_fees: 0,
        }
    }

    pub fn add_penalty(&mut self, amount: u32) {
        self.penalty_fees += amount;
        println!("{} now owes a total of: ${}", self.name, self.penalty_fees);
    }

    pub fn pay_penalty(&mut self, amount: u32) {
        if amount >= self.penalty_fees {
            println!("{} paid off all penalties: ${}", self.name, self.penalty_fees);
            self.penalty_fees = 0;
        } else {
            self.penalty_fees -= amount;
            println!("{} paid ${}. Remaining balance: ${}", self.name, amount, self.penalty_fees);
        }
    }
}
