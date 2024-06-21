#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;


fn main() {
    loop {
        // Print the prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            break; // Exit the loop if there's an error reading input
        }

        // Trim the input to remove any trailing newlines or spaces
        let command = input.trim();

        // Check for the exit command
        if command == "exit 0" {
            process::exit(0);
        }

        // Print the error message for unrecognized command
        if !command.is_empty() {
            println!("{}: command not found", command);
        }
    }
}
