use std::os::unix::fs::PermissionsExt;

pub trait Runnable {
    fn run(&self);
}

pub enum Command<'a> {
    Builtin(Builtin<'a>),
    Binary(Binary<'a>),
    Unknown(&'a str),
}

pub enum Builtin<'a> {
    Echo(&'a str),
    Exit,
    Type(TypeCommand<'a>),
    Pwd,
    Cd(Cd<'a>),
}

impl<'a> Runnable for Builtin<'a> {
    fn run(&self) {
        match self {
            Builtin::Echo(args) => {
                println!("{}", args);
            }
            Builtin::Exit => {
                std::process::exit(0);
            }
            Builtin::Type(type_cmd) => {
                type_cmd.run();
            }
            Builtin::Pwd => {
                let current_dir = std::env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            Builtin::Cd(cd) => {
                cd.run();
            }
        }
    }
}

impl<'a> Command<'a> {
    pub fn parse(input: &'a str) -> Self {
        let trimmed = input.trim();
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("").trim();
        match command {
            "echo" => Command::Builtin(Builtin::Echo(args)),
            "exit" => Command::Builtin(Builtin::Exit),
            "type" => Command::Builtin(Builtin::Type(TypeCommand { target: args })),
            "pwd" => Command::Builtin(Builtin::Pwd),
            "cd" => {
                let target = if args.is_empty() { "/home" } else { args };
                Command::Builtin(Builtin::Cd(Cd { target }))
            }
            _ => {
                if args.is_empty() {
                    Command::Unknown(command)
                } else {
                    Command::Binary(Binary::new(command, args))
                }
            }
        }
    }
}

pub struct Binary<'a> {
    path: &'a str,
    args: &'a str,
}

impl<'a> Binary<'a> {
    pub fn new(path: &'a str, args: &'a str) -> Self {
        Binary { args, path }
    }
}

impl<'a> Runnable for Binary<'a> {
    fn run(&self) {
        match std::process::Command::new(self.path)
            .args(self.args.split_whitespace())
            .output()
        {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            Err(e) => {
                eprintln!("Failed to execute {}: {}", self.path, e);
            }
        }
    }
}

pub struct TypeCommand<'a> {
    pub target: &'a str,
}

impl<'a> Runnable for TypeCommand<'a> {
    fn run(&self) {
        let target = self.target;
        match Command::parse(target) {
            Command::Builtin(_) => {
                println!("{} is a shell builtin", target);
                return;
            }
            Command::Unknown(_) | Command::Binary(_) => {}
        }

        if let Some(path) = find_in_path(target) {
            println!("{} is {}", target, path.display());
        } else {
            println!("{}: not found", target);
        }
    }
}

fn find_in_path(command: &str) -> Option<std::path::PathBuf> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    for dir in path_var.split(':') {
        let full_path = std::path::Path::new(dir).join(command);
        if is_executable(&full_path) {
            return Some(full_path);
        }
    }
    None
}

fn is_executable(path: &std::path::Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.is_file() && (m.permissions().mode() & 0o111 != 0))
        .unwrap_or(false)
}

pub struct Cd<'a> {
    pub target: &'a str,
}

impl<'a> Runnable for Cd<'a> {
    fn run(&self) {
        let path = if self.target == "~" {
            Self::get_home_dir()
        } else {
            std::path::PathBuf::from(self.target)
        };

        if path.exists() {
            if let Err(_e) = std::env::set_current_dir(&path) {
                eprintln!("cd: not a directory: {}", self.target);
            }
        } else {
            eprintln!("cd: {}: No such file or directory", self.target);
        }
    }
}

impl<'a> Cd<'a> {
    fn get_home_dir() -> std::path::PathBuf {
        std::env::var("HOME")
            .map(|home| std::path::PathBuf::from(home))
            .unwrap_or_else(|_| std::path::PathBuf::from("/home"))
    }
}
