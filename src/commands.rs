use std::os::unix::fs::PermissionsExt;

pub enum Command<'a> {
    Echo(&'a str),
    Exit,
    Type(TypeCommand<'a>),
    Binary(Binary<'a>),
    Unknown(&'a str),
}

impl<'a> Command<'a> {
    pub fn parse(input: &'a str) -> Self {
        let trimmed = input.trim();
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let command = parts.next().unwrap_or("");
        let args = parts.next().unwrap_or("").trim();

        match command {
            "echo" => Command::Echo(args),
            "exit" => Command::Exit,
            "type" => Command::Type(TypeCommand { target: args }),
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

    pub fn run(&self) {
        std::process::Command::new(self.path).args(self.args.to_string().split_whitespace());
    }
}

pub struct TypeCommand<'a> {
    pub target: &'a str,
}

impl<'a> TypeCommand<'a> {
    pub fn run(&self) {
        let target = self.target;

        match Command::parse(target) {
            Command::Echo(_) | Command::Exit | Command::Type(_) => {
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
