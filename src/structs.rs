use std::borrow::Borrow;
use serde::{Serialize, Deserialize};
use serde_json::Value::{Null, String};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub Name: String,
    pub Amount: i128,
    pub Budget: Budget,
    pub Account_Type: AccountType
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Budget {
    pub Amount_Initial: i128,
    pub Amount_Left: i128
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub Title: String,
    pub Amount: i128
}

//Account functions
pub fn GenerateAccountFromJson(accnt: String) -> Account {
    let mut ret: Account = serde_json::from_str(&accnt).unwrap();
    return ret;
}

pub fn AccountToJson(accnt: Account) -> String {
    let mut ret: String = serde_json::to_string(&accnt).unwrap();
    return ret;
}

pub fn GenerateAccountFromInput() -> Account {
    let mut nme = String::new();

    println!("vv Enter the account name vv");
    io::stdin().read_line(&mut nme);

    let mut stramnmt: String = String::from("0");

    println!("vv Enter the account amount [default is 0] vv");
    io::stdin().read_line(&mut stramnmt);

    let mut amnt: i128 = stramnmt.parse::<i128>().unwrap();

    let mut bdgt = Budget {
        Amount_Initial: 0,
        Amount_Left: 0
    };

    let mut accndtpe: AccountType = AccountType::EMPTY;
    let mut typestraccnt = String::new();

    println!("vv Enter the account type vv");
    io::stdin().read_line(&mut typestraccnt);

    match typestraccnt.to_lowercase() {
        String::from("checking") => accndtpe = AccountType::Checking,
        String::from("savings") => accndtpe = AccountType::Savings,
        String::from("retirement") => accndtpe = AccountType::Retirement,
        String::from("roth") => accndtpe = AccountType::Roth,
        _ => AccountType::EMPTY
    }

    let ret: Account = Account {
        Name: nme,
        Amount: amnt,
        Budget: bdgt,
        Account_Type: accndtpe
    };
}

//Budget Functions

//Transaction Functions

//ENUMS AND ENUM FUNCTIONS
pub enum AccountType {
    Checking,
    Savings,
    Retirement,
    Roth,
    EMPTY
}