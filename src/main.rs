#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{self, Command};

fn main() {
    let builtins = ["echo", "exit", "type"];

    loop {
        print_prompt();

        let command = read_command();
        if command.is_empty() {
            continue;
        }

        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd_name = parts[0];
        let args = &parts[1..];

        if cmd_name == "exit" && args == ["0"] {
            process::exit(0);
        }

        if let Some(builtin) = handle_builtin(cmd_name, args, &builtins) {
            println!("{}", builtin);
        } else {
            match find_in_path(cmd_name) {
                Some(path) => run_external_command(&path, args),
                None => println!("{}: command not found", cmd_name),
            }
        }
    }
}

fn print_prompt() {
    print!("$ ");
    io::stdout().flush().unwrap();
}

fn read_command() -> String {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input.trim().to_string()
    } else {
        String::new()
    }
}

fn handle_builtin(cmd_name: &str, args: &[&str], builtins: &[&str]) -> Option<String> {
    match cmd_name {
        "echo" => Some(args.join(" ")),
        "type" => {
            if args.is_empty() {
                Some("type: missing argument".to_string())
            } else {
                let cmd_to_check = args[0];
                if builtins.contains(&cmd_to_check) {
                    Some(format!("{} is a shell builtin", cmd_to_check))
                } else {
                    match find_in_path(cmd_to_check) {
                        Some(path) => Some(format!("{} is {}", cmd_to_check, path)),
                        None => Some(format!("{}: not found", cmd_to_check)),
                    }
                }
            }
        }
        _ => None,
    }
}

fn find_in_path(command: &str) -> Option<String> {
    if let Ok(path_var) = env::var("PATH") {
        for path in path_var.split(':') {
            let full_path = format!("{}/{}", path, command);
            if fs::metadata(&full_path).is_ok() {
                return Some(full_path);
            }
        }
    }
    None
}

fn run_external_command(path: &str, args: &[&str]) {
    let mut child = Command::new(path)
        .args(args)
        .spawn()
        .expect("failed to execute process");

    child.wait().expect("failed to wait on child");
}