use std::os::unix::fs::PermissionsExt;

pub trait Runnable {
    fn run(&self);
}

pub enum Command {
    Builtin(Builtin),
    Binary(Binary),
    Unknown(String),
}

pub enum Builtin {
    Echo(String),
    Exit,
    Type(TypeCommand),
    Pwd,
    Cd(Cd),
}

impl Runnable for Builtin {
    fn run(&self) {
        match self {
            Builtin::Echo(args) => println!("{}", args),
            Builtin::Exit => std::process::exit(0),
            Builtin::Type(type_cmd) => type_cmd.run(),
            Builtin::Pwd => println!("{}", std::env::current_dir().unwrap().display()),
            Builtin::Cd(cd) => cd.run(),
        }
    }
}

impl Command {
    pub fn parse(argv: Vec<String>) -> Self {
        if argv.is_empty() {
            return Command::Unknown("".into());
        }

        let command = &argv[0];
        let args = argv[1..].to_vec();

        match command.as_str() {
            "echo" => Command::Builtin(Builtin::Echo(args.join(" "))),
            "exit" => Command::Builtin(Builtin::Exit),
            "type" => {
                let target = args.get(0).cloned().unwrap_or_default();
                Command::Builtin(Builtin::Type(TypeCommand { target }))
            }
            "pwd" => Command::Builtin(Builtin::Pwd),
            "cd" => {
                let target = args.get(0).cloned().unwrap_or_else(|| "/home".to_string());
                Command::Builtin(Builtin::Cd(Cd { target }))
            }
            _ => {
                if args.is_empty() {
                    Command::Unknown(command.clone())
                } else {
                    Command::Binary(Binary::new(command.clone(), args))
                }
            }
        }
    }
}

pub struct Binary {
    path: String,
    args: Vec<String>,
}

impl Binary {
    pub fn new(path: String, args: Vec<String>) -> Self {
        Self { path, args }
    }
}

impl Runnable for Binary {
    fn run(&self) {
        match std::process::Command::new(&self.path)
            .args(&self.args)
            .output()
        {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Failed to execute {}: {}", self.path, e);
            }
        }
    }
}
pub struct TypeCommand {
    pub target: String,
}

impl Runnable for TypeCommand {
    fn run(&self) {
        let target = self.target.clone();
        match Command::parse(vec![target.clone()]) {
            Command::Builtin(_) => println!("{} is a shell builtin", target),
            _ => {
                if let Some(path) = find_in_path(&target) {
                    println!("{} is {}", target, path.display());
                } else {
                    println!("{}: not found", target);
                }
            }
        }
    }
}

fn find_in_path(command: &str) -> Option<std::path::PathBuf> {
    std::env::var("PATH").ok()?.split(':').find_map(|dir| {
        let full_path = std::path::Path::new(dir).join(command);
        if is_executable(&full_path) {
            Some(full_path)
        } else {
            None
        }
    })
}

fn is_executable(path: &std::path::Path) -> bool {
    std::fs::metadata(path)
        .map(|m| m.is_file() && (m.permissions().mode() & 0o111 != 0))
        .unwrap_or(false)
}

pub struct Cd {
    pub target: String,
}

impl Runnable for Cd {
    fn run(&self) {
        let path = if self.target == "~" {
            Cd::get_home_dir()
        } else {
            std::path::PathBuf::from(&self.target)
        };

        if path.exists() {
            if let Err(_) = std::env::set_current_dir(&path) {
                eprintln!("cd: not a directory: {}", self.target);
            }
        } else {
            eprintln!("cd: {}: No such file or directory", self.target);
        }
    }
}

impl Cd {
    fn get_home_dir() -> std::path::PathBuf {
        std::env::var("HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("/home"))
    }
}
