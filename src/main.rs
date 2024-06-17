#[allow(unused_imports)]
use std::io::{self, Write};

mod commands;

use commands::Commands;


fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut commands = Commands::new();
    commands.add_built_in_commands();


    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).unwrap() > 0 {
        let ls = input.trim().split_whitespace().collect::<Vec<&str>>();
        commands.execute_command(ls[0], ls[1..].to_vec());
        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}