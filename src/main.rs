use crate::systemhandler::{Installer};
use std::env;
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;
use serde_json::to_string;
use crate::structs::{Account, ImportAccounts};

mod systemhandler;
mod structs;
mod generichandlers;

fn main() {
    println!("Starting Clidget.rs...");

    let mut args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        if args.contains(&"install".to_string()) {
            Installer();
        }
    } else {
        MainProgram();
    }
}

pub const AccountList: Vec<Account> = ImportAccounts();

fn MainProgram() {
    let mut input: String = String::new();

    loop {
        println!("vv Clidget [help to list commands] vv");
        io::stdin().read_line(&mut input);

        match input.to_lowercase().trim() {
            "help" => ListCommands(),
            "select-account" => UpdateAccountListAndSelect(),
            _ => {}
        }

        fn UpdateAccountListAndSelect() {
            AccountList = ImportAccounts();

            let mut inp = String::new();

            println!("vv Enter Account Name to Select vv");
            io::stdin().read_line(&mut inp);

            for x in AccountList {
                let nxo = &x;

                if nxo.Name == inp {
                    AccountHandler(x);
                }
            }
        }

        input = String::from("");
    }
}

pub fn ListCommands() {
    let mut commands = vec!["help - Displays this message", "select-account (account name) - Selects an account", "create-account - Creates an account"];

    println!("--- Clidget Commands ---");
    for line in commands.iter() {
        println!("{}", line);
    }
    println!("-------------------------");
}

pub fn AccountHandler(accnt: Account) {

}
