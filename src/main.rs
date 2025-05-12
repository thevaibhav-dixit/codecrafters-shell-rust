mod commands;

use std::io::{self, Write};
use std::process::exit;

use commands::*;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user inpu
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input
            .split_whitespace()
            .next()
            .unwrap_or("")
            .parse::<Command>()
            .unwrap_or_else(|_| {
                println!("{}: command not found", input.trim());
                Command::Unknown
            });

        match command {
            Command::Echo => {
                let args: Vec<&str> = input.split_whitespace().collect();
                if args.len() > 1 {
                    println!("{}", args[1..].join(" "));
                } else {
                    println!("echo: no arguments provided");
                }
            }
            Command::Exit => {
                exit(0);
            }
            Command::Unknown => {}
        }
    }
}
