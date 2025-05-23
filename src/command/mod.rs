mod binary;
mod cd;
mod echo;
mod exit;
mod history;
mod pwd;
mod r#type;

use binary::Binary;
use cd::Cd;
use echo::Echo;
use exit::Exit;
use history::History;
use pwd::Pwd;
use r#type::Type;

pub trait Runnable {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
        history: &mut Vec<String>,
    ) -> std::io::Result<()>;
}

pub enum Command {
    Builtin(Builtin),
    Binary(Binary),
    Unknown(String),
}

impl Runnable for Command {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        match self {
            Command::Builtin(builtin) => builtin.run(args, out_writer, err_writer, history),
            Command::Binary(binary) => binary.run(args, out_writer, err_writer, history),
            Command::Unknown(s) => {
                history.push(s.to_string());
                writeln!(err_writer, "{}: command not found", args[0])
            }
        }
    }
}

impl std::str::FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.parse::<Builtin>().is_ok() {
            Ok(Command::Builtin(s.parse().unwrap()))
        } else if find_in_path(s).is_some() {
            Ok(Command::Binary(Binary::new(find_in_path(s).unwrap())))
        } else {
            Ok(Command::Unknown(s.to_string()))
        }
    }
}

pub enum Builtin {
    Echo(Echo),
    Exit(Exit),
    Type(Type),
    Pwd(Pwd),
    Cd(Cd),
    History(History),
}

impl Runnable for Builtin {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        match self {
            Builtin::Echo(echo) => echo.run(args, out_writer, err_writer, history),
            Builtin::Exit(exit) => exit.run(args, out_writer, err_writer, history),
            Builtin::Type(ty) => ty.run(args, out_writer, err_writer, history),
            Builtin::Pwd(pwd) => pwd.run(args, out_writer, err_writer, history),
            Builtin::Cd(cd) => cd.run(args, out_writer, err_writer, history),
            Builtin::History(hist) => hist.run(args, out_writer, err_writer, history),
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
            "history" => Ok(Builtin::History(History)),
            _ => Err(()),
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
    use std::os::unix::fs::PermissionsExt;

    std::fs::metadata(path)
        .map(|m| m.is_file() && (m.permissions().mode() & 0o111 != 0))
        .unwrap_or(false)
}
