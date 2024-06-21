#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Print the prompt
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    // Trim the input to remove any trailing newlines or spaces
    let command = input.trim();

    // Print the error message for unrecognized command
    println!("{}: command not found", command);
}
