#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();

        let builtins = &["exit", "echo"];
        if command == "exit" {
            break;
        } else if command.starts_with("echo") {
            println!("{}", &command[5..]);
        } else if command.starts_with("type") {
            if builtins.contains(&&command[5..]) {
                println!("{} is a shell builtin", &command[5..]);
            } else {
                println!("{}: not found", &command[5..]);
            }
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}
