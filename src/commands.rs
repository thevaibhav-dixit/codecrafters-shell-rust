use std::io::Write;
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

impl std::str::FromStr for Builtin {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "echo" => Ok(Builtin::Echo("".into())),
            "exit" => Ok(Builtin::Exit),
            "type" => Ok(Builtin::Type(TypeCommand { target: "".into() })),
            "pwd" => Ok(Builtin::Pwd),
            "cd" => Ok(Builtin::Cd(Cd { target: "".into() })),
            _ => Err(()),
        }
    }
}

impl Command {
    pub fn parse(input: (Vec<String>, Option<String>)) -> Self {
        let (argv, output_target) = input;
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
                    Command::Binary(Binary::new(command.clone(), args, output_target))
                }
            }
        }
    }

    pub fn run(&self) {
        match self {
            Command::Builtin(builtin) => builtin.run(),
            Command::Binary(binary) => binary.run(),
            Command::Unknown(cmd) => println!("{}: command not found", cmd),
        }
    }
}

pub struct Binary {
    path: String,
    args: Vec<String>,
    output_target: Option<String>,
}

impl Binary {
    pub fn new(path: String, args: Vec<String>, output_target: Option<String>) -> Self {
        Self {
            path,
            args,
            output_target,
        }
    }
}

impl Runnable for Binary {
    fn run(&self) {
        match std::process::Command::new(&self.path)
            .args(&self.args)
            .output()
        {
            Ok(output) => {
                if let Some(file_name) = self.output_target.as_ref() {
                    let mut file =
                        std::fs::File::create(file_name).expect("should be able to open the file");
                    if !output.stdout.is_empty() {
                        file.write_all(&output.stdout).unwrap();
                    }
                    if !output.stderr.is_empty() {
                        eprint!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }

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
        match target.parse::<Builtin>() {
            Ok(_) => println!("{} is a shell builtin", target),
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
