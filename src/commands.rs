pub enum Command {
    Echo,
    Exit,
    Unknown,
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim() {
            "echo" => Ok(Command::Echo),
            "exit" => Ok(Command::Exit),
            _ => Err(format!("{}: command not found", input.trim())),
        }
    }
}
