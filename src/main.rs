mod command;
mod parser;

use rustyline::{
    completion::{Completer, Pair},
    error::ReadlineError,
    highlight::{CmdKind, Highlighter, MatchingBracketHighlighter},
    hint::Hinter,
    validate::{ValidationContext, ValidationResult, Validator},
    CompletionType, Config, Editor, Helper,
};

use std::{
    borrow::Cow::{self, Borrowed},
    collections::HashSet,
    io::{self, Write},
};

use command::Runnable;
use parser::*;

struct ShellCompleter {
    commands: HashSet<String>,
    highlighter: MatchingBracketHighlighter,
}

impl ShellCompleter {
    fn new() -> Self {
        let mut commands = HashSet::new();
        commands.insert("echo ".to_string());
        commands.insert("exit ".to_string());
        ShellCompleter {
            commands,
            highlighter: MatchingBracketHighlighter::new(),
        }
    }
}

impl Helper for ShellCompleter {}

impl Completer for ShellCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> Result<(usize, Vec<Self::Candidate>), ReadlineError> {
        let line_parts: Vec<&str> = line[..pos].split_whitespace().collect();

        if line_parts.is_empty() || (line_parts.len() == 1 && !line.ends_with(' ')) {
            let prefix = line_parts.get(0).map_or("", |s| *s);

            let candidates: Vec<Pair> = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(prefix))
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();

            let start = line[..pos].rfind(' ').map_or(0, |i| i + 1);

            Ok((start, candidates))
        } else {
            Ok((pos, vec![]))
        }
    }
}

impl Highlighter for ShellCompleter {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Borrowed(hint)
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

impl Hinter for ShellCompleter {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Validator for ShellCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl = Editor::with_config(config)?;

    let completer = ShellCompleter::new();
    rl.set_helper(Some(completer));

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
            Err(ReadlineError::Interrupted) => {
                std::process::exit(0);
            }
            Err(ReadlineError::Eof) => {
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
