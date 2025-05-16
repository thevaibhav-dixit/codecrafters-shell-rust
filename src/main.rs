mod commands;
mod parser;

use std::io::{self, Write};

use commands::*;
use parser::*;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let res = Parser::parse(&input).expect("should parse");

        Command::parse(res).run();
    }
}
