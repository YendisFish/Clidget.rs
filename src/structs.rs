use serde::{Serialize, Deserialize};

pub struct Account {
    Name: String,
    Amount: i128,
    Budget: Budget,
    Account_Type: AccountType
}

pub struct Budget {
    Amount_Initial: i128,
    Amount_Left: i128
}

pub struct Transaction {

}

//Account functions

//Budget Functions

//Transaction Functions

//ENUMS AND ENUM FUNCTIONS
pub enum AccountType {
    Checking,
    Savings,
    Retirement,
    Roth
}