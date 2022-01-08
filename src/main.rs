use crate::systemhandler::{Installer};
use std::env;
use std::io;
use std::io::Write;
use std::process;
use std::process::Command;

mod systemhandler;
mod structs;
mod generichandlers;

fn main() {
    println!("Starting Clidget.rs...");

    let args: Vec<String> = env::args().collect();

    MainProgram();

    /*
    if args.len() > 0 {
        if args[0] == "Installer" {
            println!("Please run installer with sudo or it wont work correctly! Did you run with sudo?");

            let mut yorn1: String = String::new();
            io::stdin().read_line(&mut yorn1);

            if yorn1 == "yes" {
                Installer();
            }

        }
    }

    if args.len() == 0 {
        MainProgram();
    }*/
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
