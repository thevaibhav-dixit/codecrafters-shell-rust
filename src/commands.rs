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
    Echo((Vec<String>, Option<String>, Option<String>)),
    Exit,
    Type(TypeCommand),
    Pwd,
    Cd(Cd),
}

impl Runnable for Builtin {
    fn run(&self) {
        match self {
            Builtin::Echo((args, output_target, stderr_target)) => {
                let output = args.join(" ") + "\n";

                if let Some(file_name) = output_target {
                    if let Some(parent) = std::path::Path::new(file_name).parent() {
                        std::fs::create_dir_all(parent).unwrap();
                    }
                    let mut file =
                        std::fs::File::create(file_name).expect("could not open stdout file");
                    file.write_all(output.as_bytes()).unwrap();
                } else {
                    print!("{}", output);
                }

                if let Some(file_name) = stderr_target {
                    if let Some(parent) = std::path::Path::new(file_name).parent() {
                        std::fs::create_dir_all(parent).unwrap();
                    }
                    std::fs::File::create(file_name).expect("could not open stderr file");
                }
            }
            Builtin::Exit => std::process::exit(0),
            Builtin::Type(type_cmd) => type_cmd.run(),
            Builtin::Pwd => println!("{}", std::env::current_dir().unwrap().display()),
            Builtin::Cd(cd) => cd.run(),
        }
    }
}

// this is wrong.... will need to be fixed
impl std::str::FromStr for Builtin {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "echo" => Ok(Builtin::Echo((Vec::new(), None, None))),
            "exit" => Ok(Builtin::Exit),
            "type" => Ok(Builtin::Type(TypeCommand { target: "".into() })),
            "pwd" => Ok(Builtin::Pwd),
            "cd" => Ok(Builtin::Cd(Cd { target: "".into() })),
            _ => Err(()),
        }
    }
}

impl Command {
    pub fn parse(input: (Vec<String>, Option<String>, Option<String>)) -> Self {
        let (argv, stdout_target, stderr_target) = input;
        if argv.is_empty() {
            return Command::Unknown("".into());
        }

        let command = &argv[0];
        let args = argv[1..].to_vec();

        match command.as_str() {
            "echo" => Command::Builtin(Builtin::Echo((args, stdout_target, stderr_target))),
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
                    Command::Binary(Binary::new(
                        command.clone(),
                        args,
                        stdout_target,
                        stderr_target,
                    ))
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
    stderr_target: Option<String>,
}

impl Binary {
    pub fn new(
        path: String,
        args: Vec<String>,
        output_target: Option<String>,
        stderr_target: Option<String>,
    ) -> Self {
        Self {
            path,
            args,
            output_target,
            stderr_target,
        }
    }
}

impl Runnable for Binary {
    fn run(&self) {
        if let Some(file_name) = &self.output_target {
            if let Some(parent) = std::path::Path::new(file_name).parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::File::create(file_name).unwrap();
        }

        if let Some(file_name) = &self.stderr_target {
            if let Some(parent) = std::path::Path::new(file_name).parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            std::fs::File::create(file_name).unwrap();
        }

        match std::process::Command::new(&self.path)
            .args(&self.args)
            .output()
        {
            Ok(output) => {
                if let Some(file_name) = &self.output_target {
                    let mut file = std::fs::File::create(file_name).expect("cannot write stdout");
                    file.write_all(&output.stdout).unwrap();
                } else {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }

                if let Some(file_name) = &self.stderr_target {
                    let mut file = std::fs::File::create(file_name).expect("cannot write stderr");
                    file.write_all(&output.stderr).unwrap();
                } else {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }

            Err(e) => {
                let msg = format!("{}: {}\n", self.path, e);

                if let Some(file_name) = &self.stderr_target {
                    let mut file = std::fs::File::create(file_name).expect("cannot write stderr");
                    file.write_all(msg.as_bytes()).unwrap();
                } else {
                    eprint!("{}", msg);
                }
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
