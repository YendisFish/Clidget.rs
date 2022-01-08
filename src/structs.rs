use serde::{Serialize, Deserialize};

pub struct Account {
    pub Name: String,
    pub Amount: i128,
    pub Budget: Budget,
    pub Account_Type: AccountType
}

pub struct Budget {
    pub Amount_Initial: i128,
    pub Amount_Left: i128
}

pub struct Transaction {
    pub Title: String,
    pub Amount: i128
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