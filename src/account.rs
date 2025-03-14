pub struct Account {
    pub account_id: String,
    pub user_id: String,
    pub balance: u32,
}

impl Account {
    pub fn new(user_id: String, balance: u32, account_id: String) -> Self {
        Self {
            user_id,
            balance,
            account_id,
        }
    }

    pub fn get_balance(&self) -> u32 {
        self.balance
    }

    pub fn take_money(&mut self, amount: u32) -> Result<u32, String> {
        if amount > self.balance {
            Err(String::from("Insufficient balance!"))
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    pub fn add_money(&mut self, amount: u32) -> u32 {
        self.balance += amount;
        self.balance
    }

    pub fn transfer_money(&mut self, amount: u32, receiver: &mut Account) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient balance!".to_string());
        }
        self.balance -= amount;
        receiver.balance += amount;

        Ok(())
    }
}
