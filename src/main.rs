#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

enum Command<'a> {
    Exit,
    Echo(&'a str),
    Type(&'a str),
    Empty,
    Unknown(&'a str),
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Self {
        let input = input.trim();

        let (cmd, args) = input.split_once(" ").unwrap_or((input, ""));
        let args = args.trim();

        match cmd {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "type" => Command::Type(args),
            "" => Command::Empty,
            _ => Command::Unknown(cmd),
        }
    }

    fn execute(&self) -> bool {
        match self {
            Command::Exit => return false,
            Command::Echo(args) => println!("{}", args),
            Command::Type(cmd) => match Command::parse(cmd) {
                Command::Empty => {
                    println!("{}: not found", cmd);
                }
                Command::Unknown(cmd) => match self.find_executable(cmd) {
                    Some(path) => println!("{} is {}", cmd, path.display()),
                    None => println!("{} not found", cmd),
                },
                _ => {
                    println!("{} is a shell builtin", cmd);
                }
            },
            Command::Unknown(cmd) => println!("{}: command not found", cmd),
            Command::Empty => println!(": command not found"),
        }
        true
    }

    fn find_executable(&self, cmd: &str) -> Option<PathBuf> {
        let paths = std::env::var("PATH").ok()?;

        for mut path in env::split_paths(&paths) {
            path.push(cmd);

            if path.is_file() && is_executable(&path) {
                return Some(path);
            }
        }
        None
    }
}

fn is_executable(path: &Path) -> bool {
    let Ok(metadata) = path.metadata() else {
        return false;
    };

    if !metadata.is_file() {
        return false;
    }

    let mode = metadata.permissions().mode();

    mode & 0o111 != 0
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Command::parse(&input);
        if !command.execute() {
            break;
        }
    }
}
