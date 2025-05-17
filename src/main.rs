mod command;
mod parser;

use std::io::{self, Write};

use command::Runnable;
use parser::*;

fn main() -> Result<(), std::io::Error> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let ParseOutput {
            args,
            out_target,
            err_target,
        } = Parser::parse(&input)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

        let mut out_writer = if let Some(target) = out_target {
            let file = std::fs::File::create(&target)?;
            Box::new(file) as Box<dyn Write>
        } else {
            Box::new(io::stdout())
        };

        let mut err_writer = if let Some(target) = err_target {
            let file = std::fs::File::create(&target)?;
            Box::new(file) as Box<dyn Write>
        } else {
            Box::new(io::stderr())
        };

        if let Some(cmd) = args.first() {
            if let Ok(cmd) = cmd.parse::<command::Command>() {
                let _ = cmd.run(args, &mut out_writer, &mut err_writer);
            } else {
                writeln!(err_writer, "Error: Invalid command")?
            }
        }
    }
}
