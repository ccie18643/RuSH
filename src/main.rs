use std::io::stdin;
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

fn main() {
    loop {
        let mut command_line = String::new();
        stdin().read_line(&mut command_line).unwrap();
        if command_line.trim().is_empty() {
            continue;
        }

        let (command, arguments) = parse_command_line(command_line);

        Command::new(command).args(arguments).spawn().unwrap();
    }
}
