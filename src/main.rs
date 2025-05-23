mod command;
mod input;
mod parser;

use std::io::{self, Write};

use command::Runnable;
use input::*;
use parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = ReadLine::new();
    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(input) => {
                let ParseOutput {
                    args,
                    out_target,
                    err_target,
                } = Parser::parse(&input)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

                let mut out_writer = if let Some((target, append)) = out_target {
                    let file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(append)
                        .open(&target)?;
                    Box::new(file) as Box<dyn Write>
                } else {
                    Box::new(io::stdout())
                };

                let mut err_writer = if let Some((target, append)) = err_target {
                    let file = std::fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(append)
                        .open(&target)?;
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
            Err(rustyline::error::ReadlineError::Interrupted) => {
                std::process::exit(0);
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
