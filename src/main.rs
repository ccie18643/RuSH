use std::env;
use std::io;
use std::io::Write;
use std::process;

fn print_command_prompt() {
    let current_dir = env::current_dir().unwrap();
    print!("<RuSH> [{}]$ ", current_dir.display());
    io::stdout().flush().unwrap();
}

fn parse_command(cmd: &str) -> (&str, Vec<&str>) {
    let mut tokens = cmd.split_whitespace();
    (tokens.next().unwrap_or(""), tokens.collect())
}

fn execute_command(cmd_str: &str, cmd_args: Vec<&str>) {
    let mut child = match process::Command::new(cmd_str).args(cmd_args).spawn() {
        Ok(child) => child,
        Err(_) => {
            println!("-rush: {}: command not found", cmd_str);
            return;
        }
    };
    child.wait().unwrap();
}

fn parse_command_group(cmd_grp: &str) -> Vec<(&str, Vec<&str>)> {
    let mut cmds = Vec::new();
    for token in cmd_grp.split('|') {
        cmds.push(parse_command(token));
    }
    cmds
}

fn execute_command_group(cmds: Vec<(&str, Vec<&str>)>) {
    for (cmd_str, cmd_args) in cmds {
        match cmd_str {
            "exit" => process::exit(0),
            _ => execute_command(cmd_str, cmd_args),
        }
    }
}

fn main() {
    loop {
        print_command_prompt();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().is_empty() {
            continue;
        }

        let cmds = parse_command_group(&input);
        execute_command_group(cmds);
    }
}
