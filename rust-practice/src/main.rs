use core::fmt;
use std::fmt::{Display, Formatter};

struct Bank {
    accounts: Vec<Account>,
}

struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: \n{}: {}", 
            self.holder, self.id, self.balance)
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for account in self.accounts.iter() {
            write!(f, "{}", account).unwrap();
        };
        write!(f,"")
    }
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![]}
    }
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account { id, balance: 0, holder}
    }
}

fn print_account(account: Account){
    println!("{}", account)
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1,String::from("Ellie"));

    println!("{}", bank);
    print_account(account);
    print_account(account);
}
