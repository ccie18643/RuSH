use std::env;
use std::io;
use std::io::Write;
use std::process;

fn parse_command(command: String) -> (String, Vec<String>) {
    let mut tokens = command.split_whitespace();
    let cmd = match tokens.next() {
        Some(cmd) => cmd.to_string(),
        None => String::new(),
    };
    let args = tokens.map(|t| t.to_string()).collect();
    (cmd, args)
}

fn print_command_prompt() {
    let current_dir = env::current_dir().unwrap();
    print!("[{}]$ ", current_dir.display());
    io::stdout().flush().unwrap();
}

fn execute_external_command(cmd: String, args: Vec<String>) {
    let mut child = process::Command::new(cmd).args(args).spawn().unwrap();
    child.wait().unwrap();
}

fn main() {
    println!("\n*** Entering RuSH world *** \n");

    loop {
        print_command_prompt();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            continue;
        }

        let (cmd, args) = parse_command(input);

        match cmd.as_str() {
            "exit" => break,
            _ => execute_external_command(cmd, args),
        }
    }

    println!("\n*** Exiting RuSH world *** \n");
}
