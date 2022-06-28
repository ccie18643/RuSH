use std::env::current_dir;
use std::io;
use std::io::stdin;
use std::io::Write;
use std::process::Command;

fn parse_command_line(command_line: String) -> (String, Vec<String>) {
    let mut tokens = command_line.trim().split_whitespace();
    let command = match tokens.next() {
        Some(command) => command.to_string(),
        None => String::new(),
    };
    let arguments = tokens.map(|t| t.to_string()).collect();
    (command, arguments)
}

fn print_command_prompt() {
    let current_dir = current_dir().unwrap();
    print!("[{}]$ ", current_dir.display());
    io::stdout().flush().unwrap();
}

fn execute_external_command(command: String, arguments: Vec<String>) {
    let mut child = Command::new(command).args(arguments).spawn().unwrap();
    child.wait().unwrap();
}

fn main() {
    loop {
        print_command_prompt();
        let mut command_line = String::new();
        stdin().read_line(&mut command_line).unwrap();
        if command_line.trim().is_empty() {
            continue;
        }

        let (command, arguments) = parse_command_line(command_line);

        match command.as_str() {
            "exit" => return,
            _ => execute_external_command(command, arguments),
        }
    }
}
