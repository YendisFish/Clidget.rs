use crate::systemhandler::{GetOs, Installer};
use std::env;
use std::io;

mod systemhandler;
mod structs;

pub const CURRENT_OS: String = GetOs();

fn main() {
    println!("Starting Clidget.rs...");

    let args: Vec<String> = env::args().collect();

    if args.len() < 0 {
        if args[0] == "Installer" {
            println!("Please run installer with sudo or it wont work correctly! Did you run with sudo?");

            let mut yorn1: String = String::new();
            io::stdin().read_line(&mut yorn);

            if yorn1 == "yes" {
                await!(Installer());
            }

        }
    }

    if args.len() = 0 {

    }
}

fn MainProgram() {
    let mut input: String = String::new();

    loop {
        //Add Logic
    }
}
