mod commands;

use std::io::{self, Write};
use std::process::exit;

use commands::*;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Command::parse(&input);
        match command {
            Command::Echo(args) => {
                println!("{}", args);
            }
            Command::Exit => {
                exit(0);
            }
            Command::Type(type_command) => {
                type_command.run();
            }
            Command::Binary(binary) => {
                binary.run();
            }
            Command::Pwd => {
                let current_dir = std::env::current_dir().unwrap();
                println!("{}", current_dir.display());
            }
            Command::Unknown(command) => {
                println!("{}: command not found", command);
            }
        }
    }
}
