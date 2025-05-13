mod commands;
use commands::*;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Command::parse(&input);

        match command {
            Command::Builtin(builtin) => match builtin {
                _ => builtin.run(),
            },
            Command::Binary(binary) => binary.run(),
            Command::Unknown(cmd) => {
                println!("{}: command not found", cmd);
            }
        }
    }
}
