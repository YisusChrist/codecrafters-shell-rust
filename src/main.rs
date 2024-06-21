#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{self, Command};

fn main() {
    let builtins: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

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

        match handle_builtin(cmd_name, args, &builtins) {
            Some(output) => {
                // If command is cd don't print anything
                if cmd_name == "cd" && output.is_empty() {
                    continue;
                }
                println!("{}", output)
            }
            None => {
                if let Some(path) = find_in_path(cmd_name) {
                    run_external_command(&path, args);
                } else {
                    println!("{}: command not found", cmd_name);
                }
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
        "pwd" => match env::current_dir() {
            Ok(path) => Some(path.display().to_string()),
            Err(_) => Some("pwd: could not retrieve current directory".to_string()),
        },
        "cd" => {
            if args.is_empty() {
                Some("cd: missing argument".to_string())
            } else {
                let new_dir = args[0];
                change_directory(new_dir)
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

fn change_directory(new_dir: &str) -> Option<String> {
    let current_dir = env::current_dir().ok()?;
    let mut new_path = PathBuf::from(&current_dir);

    if new_dir.starts_with('/') {
        // Absolute path
        new_path = PathBuf::from(new_dir);
    } else {
        // Relative path
        new_path.push(new_dir);
    }

    if let Err(_) = env::set_current_dir(&new_path) {
        return Some(format!("cd: {}: No such file or directory", new_dir));
    }

    None
}
