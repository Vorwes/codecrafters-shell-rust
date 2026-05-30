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
    Pwd,
    Cd(&'a str),
    Empty,
    Unknown(&'a str, &'a str),
}

impl<'a> Command<'a> {
    fn parse(input: &'a str) -> Self {
        let input = input.trim();

        let (cmd, args) = input.split_once(char::is_whitespace).unwrap_or((input, ""));
        let args = args.trim();

        match cmd {
            "exit" => Command::Exit,
            "echo" => Command::Echo(args),
            "type" => Command::Type(args),
            "pwd" => Command::Pwd,
            "cd" => Command::Cd(args),
            "" => Command::Empty,
            _ => Command::Unknown(cmd, args),
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
                Command::Unknown(cmd, _) => match find_executable(cmd) {
                    Some(path) => println!("{} is {}", cmd, path.display()),
                    None => println!("{} not found", cmd),
                },
                _ => {
                    println!("{} is a shell builtin", cmd);
                }
            },
            Command::Pwd => println!("{}", env::current_dir().unwrap().display()),
            Command::Cd(path) => {
                env::set_current_dir(path).unwrap_or_else(|_| {
                    println!("cd: no such file or directory: {}", path);
                });
            }
            Command::Unknown(cmd, args) => match find_executable(cmd) {
                Some(path) => {
                    let args: Vec<String> =
                        args.split_whitespace().map(|s| s.to_string()).collect();
                    let output = execute_external_command(&path.to_string_lossy(), &args);
                    print!("{}", output);
                }
                None => println!("{}: command not found", cmd),
            },
            Command::Empty => {}
        }
        true
    }
}

fn find_executable(cmd: &str) -> Option<PathBuf> {
    let paths = std::env::var("PATH").ok()?;

    for mut path in env::split_paths(&paths) {
        path.push(cmd);

        if is_executable(&path) {
            return Some(path);
        }
    }
    None
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

fn execute_external_command(cmd: &str, args: &[String]) -> String {
    let cmd = Path::new(cmd)
        .file_name()
        .and_then(|s| s.to_str())
        .expect("Invalid command path");

    let output = std::process::Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute command");

    String::from_utf8_lossy(&output.stdout).to_string()
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
