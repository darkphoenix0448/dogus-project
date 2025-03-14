use crate::account::Account;

pub struct User {
    pub name: String,
    pub surname: String,
    pub user_id: String,
    pub accounts: Vec<Account>,
}

impl User {
    pub fn new(name: String, surname: String, user_id: String) -> Self {
        Self {
            name,
            surname,
            user_id,
            accounts: vec![],
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn print_accounts(&self) {
        println!("Accounts of {} {}:", self.name, self.surname);
        for (i, account) in self.accounts.iter().enumerate() {
            println!("  {}. Account - Balance: {}", i + 1, account.get_balance());
        }
    }

    pub fn get_account(&self, account_id: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|account| account.account_id == account_id)
    }
}
