mod binary;
mod cd;
mod echo;
mod exit;
mod pwd;
mod r#type;

use binary::Binary;
use cd::Cd;
use echo::Echo;
use exit::Exit;
use pwd::Pwd;
use r#type::Type;

pub trait Runnable {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    );
}

pub enum Command {
    Builtin(Builtin),
    Binary(Binary),
    Unknown,
}

impl Runnable for Command {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        match self {
            Command::Builtin(builtin) => builtin.run(args, out_writer, err_writer),
            Command::Binary(binary) => binary.run(args, out_writer, err_writer),
            Command::Unknown => {
                writeln!(err_writer, "command not found").unwrap();
            }
        }
    }
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.parse::<Builtin>().is_ok() {
            Ok(Command::Builtin(s.parse().unwrap()))
        } else if check_executable(s) {
            Ok(Command::Binary(Binary::new(s.to_string())))
        } else {
            Ok(Command::Unknown)
        }
    }
}

pub enum Builtin {
    Echo(Echo),
    Exit(Exit),
    Type(Type),
    Pwd(Pwd),
    Cd(Cd),
}

impl Runnable for Builtin {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        match self {
            Builtin::Echo(echo) => echo.run(args, out_writer, err_writer),
            Builtin::Exit(exit) => exit.run(args, out_writer, err_writer),
            Builtin::Type(ty) => ty.run(args, out_writer, err_writer),
            Builtin::Pwd(pwd) => pwd.run(args, out_writer, err_writer),
            Builtin::Cd(cd) => cd.run(args, out_writer, err_writer),
        }
    }
}

impl std::str::FromStr for Builtin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "echo" => Ok(Builtin::Echo(Echo)),
            "exit" => Ok(Builtin::Exit(Exit)),
            "type" => Ok(Builtin::Type(Type)),
            "pwd" => Ok(Builtin::Pwd(Pwd)),
            "cd" => Ok(Builtin::Cd(Cd)),
            _ => Err(()),
        }
    }
}

fn check_executable(path: &str) -> bool {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    if let Ok(metadata) = fs::metadata(path) {
        if metadata.is_file() {
            let permissions = metadata.permissions();
            return permissions.mode() & 0o111 != 0; // Check if executable bit is set
        }
    }
    false
}
