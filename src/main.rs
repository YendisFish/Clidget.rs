use crate::systemhandler::{Installer};
use std::env;
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;
use serde_json::to_string;

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

fn MainProgram() {
    let mut input: String = String::new();

    loop {
        println!("vv Clidget [list to list commands] vv");
        io::stdin().read_line(&mut input);

        match input.to_lowercase().trim() {
            "list" => ListCommands(),
            "select-account" => println!("PUT FUNCTION HERE"),
            _ => {}
        }

        input = String::from("");
    }
}

pub fn ListCommands() {
    let mut commands = vec!["list - Displays this message", "select-account (account name) - Selects an account", "create-account - Creates an account"];

    println!("--- Clidget Commands ---");
    for line in commands.iter() {
        println!("{}", line);
    }
    println!("-------------------------");
}
