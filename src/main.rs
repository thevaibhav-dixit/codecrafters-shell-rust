#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user inpu
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        println!("{}: command not found", input.trim());
    }
}
