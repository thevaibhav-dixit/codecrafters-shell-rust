mod command;
mod input;
mod parser;

use std::io::{self, Write};

use command::Runnable;
use input::*;
use parser::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut history = Vec::new();
    let mut rl = ReadLine::new();
    loop {
        let readline = rl.readline("$ ");

        match readline {
            Ok(input) => {
                rl.add_history(&input); // add own readline implmentation in future to remove deps
                                        // on rustyline

                let ParseOutput {
                    commands,
                    out_target,
                    err_target,
                } = Parser::parse(&input)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

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

                let mut input: Box<dyn std::io::Read> = Box::new(io::empty());

                for (i, args) in commands.iter().enumerate() {
                    let mut out_writer: Box<dyn Write> = if i == commands.len() - 1 {
                        if let Some((ref target, append)) = out_target {
                            let file = std::fs::OpenOptions::new()
                                .write(true)
                                .create(true)
                                .append(append)
                                .open(&target)?;
                            Box::new(file)
                        } else {
                            Box::new(io::stdout())
                        }
                    } else {
                        let (reader, writer) = os_pipe::pipe()?;
                        input = Box::new(reader);
                        Box::new(writer)
                    };

                    if let Some(cmd) = args.first() {
                        if let Ok(cmd) = cmd.parse::<command::Command>() {
                            if i == 0 {
                                cmd.run(
                                    args.clone(),
                                    None,
                                    &mut out_writer,
                                    &mut err_writer,
                                    &mut history,
                                )?;
                            } else {
                                cmd.run(
                                    args.clone(),
                                    Some(&mut input),
                                    &mut out_writer,
                                    &mut err_writer,
                                    &mut history,
                                )?;
                            }
                        } else {
                            writeln!(err_writer, "Error: Invalid command")?;
                            break;
                        }
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
