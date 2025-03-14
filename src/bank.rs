use crate::account::Account;
use crate::user::User;

pub struct Bank {
    pub bank_name: String,
    pub bank_balance: u128,
    pub users: Vec<User>,
}

impl Bank {
    pub fn new(bank_name: String, bank_balance: u128, users: Vec<User>) -> Self {
        Self {
            bank_name,
            bank_balance,
            users,
        }
    }

    pub fn get_bank_balance(&self) -> u128 {
        self.bank_balance
    }

    pub fn print_users(&self) {
        for user in &self.users {
            println!(
                "Name: {}, Surname: {}, ID: {}",
                user.name, user.surname, user.user_id
            );
            user.print_accounts();
        }
    }

    pub fn get_user(&self, user_id: &str) -> Option<&User> {
        self.users.iter().find(|user| user.user_id == user_id)
    }

    pub fn get_account_by_id(&self, account_id: &str) -> Option<&Account> {
        for user in &self.users {
            if let Some(account) = user.get_account(account_id) {
                return Some(account);
            }
        }
        None
    }
}
