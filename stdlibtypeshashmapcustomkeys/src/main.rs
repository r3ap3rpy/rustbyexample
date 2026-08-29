use std::collections::HashMap;

#[derive(Eq,Hash,PartialEq)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>,AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}",username);
    println!("Password: {}",password);
    println!("Attempting login.");
    let logon = Account {
        username, password
    };
    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successfull login!");
            println!("Name: {}",account_info.name);
            println!("Email: {}",account_info.email);
        },
        _ => println!("Logon failed!"),
    }

}

fn main() {
    let mut accounts: Accounts = HashMap::new();
    let account = Account {
        username: "r3ap3rpy",
        password: "start!12345",
    };
    let account_info = AccountInfo {
        name: "Szabó Dániel Ernő",
        email: "r3ap3rpy@gmail.com",
    };
    accounts.insert(account, account_info);
    try_logon(&accounts, "r3ap3rpy","start!12345");
    try_logon(&accounts, "admin","admin");
}
