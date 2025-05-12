#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user inpu
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            exit(0)
        }
        println!("{}: command not found", input.trim());
    }
}
