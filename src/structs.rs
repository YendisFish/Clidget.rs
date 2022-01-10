use std::borrow::Borrow;
use serde::{Serialize, Deserialize};
use std::io;
use std::fs;
use std::path::PathBuf;

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
pub fn AddBudgetToAccount(accnt: Account, budget: Budget) {
    let mut accnttochange = accnt;

    let ret: Account = Account {
      Name: accnttochange.Name,
      Amount: accnttochange.Amount,
      Budget: budget,
      Account_Type: accnttochange.Account_Type
    };

    UpdateAccount(ret);
}

pub fn ProcessTransaction(accnt: Account, tr: Transaction) {
    let mut toProcess = &tr;

    let mut newBal: i128 = accnt.Amount - toProcess.Amount;

    let mut ret: Account = Account {
        Name: accnt.Name,
        Amount: newBal,
        Budget: accnt.Budget,
        Account_Type: accnt.Account_Type
    };

    UpdateAccount(ret);
}

pub fn ImportAccounts() -> Vec<Account>{
    let mut dirFiles= fs::read_dir("/etc/Clidget/Core/Accounts/").unwrap();
    let mut dirStr: Vec<String> = Vec::new();

    for dir in dirFiles {
        dirStr.push(String::from(&dir.unwrap().path().into_os_string().into_string().unwrap()));
    }

    let mut accounts: Vec<Account> = Vec::new();

    for dirname in dirStr {
        for file in fs::read_dir(dirname).unwrap() {
            if String::from(&file.as_ref().unwrap().file_name().to_string_lossy().to_string()).contains(".json") {
                let mut toRead = String::from(file.unwrap().path().to_string_lossy().to_string());
                let mut toGen = fs::read_to_string(toRead).unwrap();

                accounts.push(GenerateAccountFromJson(toGen));
            }
        }
    }

    return accounts;
}

pub fn UpdateAccount(accnt: Account) {
    fs::remove_dir("/etc/Clidget/Core/Accounts".to_string() + &accnt.Name + &"/" + &accnt.Name + "/");

    CreateAccount(&accnt)
}

pub fn CreateAccount(accnttocreate: &Account) {
    let parsed: String = AccountToJson(&accnttocreate);
    let mut dirStr: String = "/etc/Clidget/Core/Accounts/".to_string() + &accnttocreate.Name + &"/" + &accnttocreate.Name + &"/".to_string() + &".json";

    fs::create_dir_all(&dirStr).expect("ran into error");

    fs::write(dirStr.to_string(), &parsed);
}

pub fn GenerateAccountFromJson(accnt: String) -> Account {
    let mut ret: Account = serde_json::from_str(&accnt).unwrap();
    return ret;
}

pub fn AccountToJson(accnt: &Account) -> String {
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

    let one = String::from("checking");
    let two = String::from("savings");
    let three = String::from("retirement");
    let four = String::from("roth");

    match typestraccnt.to_lowercase() {
        one => accndtpe = AccountType::Checking,
        two => accndtpe = AccountType::Savings,
        three => accndtpe = AccountType::Retirement,
        four => accndtpe = AccountType::Roth,
        _ => {}
    }

    let ret: Account = Account {
        Name: nme,
        Amount: amnt,
        Budget: bdgt,
        Account_Type: accndtpe
    };

    return ret;
}

//Budget Functions

//Transaction Functions

//ENUMS AND ENUM FUNCTIONS
#[derive(Serialize, Deserialize, Debug)]
pub enum AccountType {
    Checking,
    Savings,
    Retirement,
    Roth,
    EMPTY
}