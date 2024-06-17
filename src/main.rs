#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    while stdin.read_line(&mut input).unwrap() > 0 {
        match input.trim() {
            "exit" => return,
            _ => println!("{}: command not found", input.trim()),
        }

        print!("$ ");
        io::stdout().flush().unwrap();
        input.clear();
    }
}
