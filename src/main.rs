mod commands;

use std::io::{self, Write};

use commands::*;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = parse_input(&input)
            .expect("Failed to parse input")
            .join(" ");

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

fn parse_input(input: &str) -> Result<Vec<String>, shellish_parse::ParseError> {
    shellish_parse::parse(input, shellish_parse::ParseOptions::default())
}
