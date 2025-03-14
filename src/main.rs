mod account;
mod bank;
mod user;

use account::Account;
use bank::Bank;
use std::io;
use user::User;

fn main() {
    println!("Welcome!");

    // Bank A users
    let mut user1 = User::new("John".to_string(), "Doe".to_string(), "1".to_string());
    let mut user2 = User::new("Jane".to_string(), "Smith".to_string(), "2".to_string());

    // Bank B users
    let mut user3 = User::new(
        "Michael".to_string(),
        "Johnson".to_string(),
        "3".to_string(),
    );
    let mut user4 = User::new("Emily".to_string(), "Davis".to_string(), "4".to_string());

    // Bank C users
    let mut user5 = User::new("David".to_string(), "Brown".to_string(), "5".to_string());
    let mut user6 = User::new("Sarah".to_string(), "Wilson".to_string(), "6".to_string());

    // Add accounts to users
    user1.add_account(Account::new(user1.user_id.clone(), 5000, "101".to_string()));
    user1.add_account(Account::new(user1.user_id.clone(), 7000, "102".to_string()));

    user2.add_account(Account::new(
        user2.user_id.clone(),
        10000,
        "201".to_string(),
    ));
    user2.add_account(Account::new(
        user2.user_id.clone(),
        100000,
        "202".to_string(),
    ));

    user3.add_account(Account::new(
        user3.user_id.clone(),
        10300,
        "301".to_string(),
    ));
    user4.add_account(Account::new(
        user4.user_id.clone(),
        15000,
        "401".to_string(),
    ));

    user5.add_account(Account::new(
        user5.user_id.clone(),
        20000,
        "501".to_string(),
    ));
    user6.add_account(Account::new(
        user6.user_id.clone(),
        25000,
        "601".to_string(),
    ));

    let account1 = Account::new("1".to_string(), 1000, "701".to_string());
    let account2 = Account::new("2".to_string(), 1000, "702".to_string());

    // Create banks
    let bank_a = Bank::new("Bank A".to_string(), 1000000000000000, vec![user1, user2]);
    let bank_b = Bank::new("Bank B".to_string(), 1000000000000000, vec![user3, user4]);
    let bank_c = Bank::new("Bank C".to_string(), 1000000000000000, vec![user5, user6]);

    // User input for menu
    let mut input = String::new();
    println!("Enter your user ID:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let user_id = input.trim().to_string();

    let (user, bank) = match bank_a.get_user(&user_id) {
        Some(user) => (user, &bank_a),
        None => match bank_b.get_user(&user_id) {
            Some(user) => (user, &bank_b),
            None => match bank_c.get_user(&user_id) {
                Some(user) => (user, &bank_c),
                None => {
                    println!("User not found");
                    return;
                }
            },
        },
    };

    loop {
        println!("Menu:");
        println!("1. Check balance");
        println!("2. Transfer money");
        println!("3. Exit");

        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice = input.trim();

        match choice {
            "1" => {
                user.print_accounts();
            }
            "2" => {
                println!("Enter your account ID:");
                input.clear();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let account_id = input.trim().to_string();

                if let Some(sender_account) = user.get_account(&account_id) {
                    println!("Enter recipient account ID:");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let recipient_account_id = input.trim().to_string();

                    let recipient_account = match bank.get_account_by_id(&recipient_account_id) {
                        Some(account) => account,
                        None => {
                            println!("Recipient account not found");
                            return;
                        }
                    };

                    println!("Enter amount to transfer:");
                    input.clear();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let amount: u32 = input.trim().parse().expect("Please enter a valid number");

                    let fee = if bank.get_user(&user_id).is_some()
                        && bank.get_user(&recipient_account.user_id).is_some()
                    {
                        0
                    } else {
                        4
                    };

                    match sender_account.transfer_money(amount, &recipient_account) {
                        Ok(()) => {
                            if fee > 0 {
                                sender_account
                                    .take_money(fee)
                                    .expect("Failed to deduct fee");
                            }
                            println!("Transfer successful with fee: {} TL", fee);
                        }
                        Err(error_msg) => println!("Error: {}", error_msg),
                    }
                    println!("Sender account balance: {}", sender_account.get_balance());
                    println!(
                        "Receiver account balance: {}",
                        recipient_account.get_balance()
                    );
                } else {
                    println!("Account not found");
                }
            }
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}
