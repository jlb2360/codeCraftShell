#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;

use commands::exit_command;

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).unwrap() > 0 {
        let ls = input.trim().split_whitespace().collect::<Vec<&str>>();
        match ls[0] {
            "exit" => exit_command(ls),
            _ => println!("{}: command not found", ls[0]),
        }

        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}