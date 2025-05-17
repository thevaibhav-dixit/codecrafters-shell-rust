use std::iter::Peekable;
use std::str::Chars;

/// Represents the various states the parser can be in
#[derive(Debug)]
enum ParseState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
}

#[derive(Debug)]
/// A `Parser` struct that holds the state and context for parsing operations.
pub struct Parser<'a> {
    args: Vec<String>,
    current: String,
    state: ParseState,
    chars: Peekable<Chars<'a>>,
}

pub struct ParseOutput {
    pub args: Vec<String>,
    pub out_target: Option<(String, bool)>,
    pub err_target: Option<String>,
}
impl Parser<'_> {
    fn new(input: &str) -> Parser {
        Parser {
            args: Vec::new(),
            current: String::new(),
            state: ParseState::Normal,
            chars: input.trim().chars().peekable(),
        }
    }

    pub fn parse(input: &str) -> Result<ParseOutput, String> {
        let mut parser = Parser::new(input);
        while let Some(ch) = parser.chars.next() {
            parser.state = match parser.state {
                ParseState::Normal => parser.handle_normal(ch)?,
                ParseState::InSingleQuote => parser.handle_in_single_quote(ch),
                ParseState::InDoubleQuote => parser.handle_in_double_quote(ch)?,
            }
        }

        if !parser.current.is_empty() {
            parser.args.push(parser.current.clone());
        }

        let res = parser.handle_redirections();
        Ok(res)
    }

    fn handle_normal(&mut self, ch: char) -> Result<ParseState, String> {
        match ch {
            '\\' => {
                // Escape the next character if present.
                if let Some(escaped) = self.chars.next() {
                    self.current.push(escaped);
                } else {
                    return Err("Trailing backslash".into());
                }
                Ok(ParseState::Normal)
            }
            '\'' => Ok(ParseState::InSingleQuote),
            '"' => Ok(ParseState::InDoubleQuote),
            c if c.is_whitespace() => {
                if !self.current.is_empty() {
                    self.args.push(std::mem::take(&mut self.current));
                }
                Ok(ParseState::Normal)
            }
            _ => {
                self.current.push(ch);
                Ok(ParseState::Normal)
            }
        }
    }

    fn handle_in_single_quote(&mut self, ch: char) -> ParseState {
        if ch == '\'' {
            ParseState::Normal
        } else {
            self.current.push(ch);
            ParseState::InSingleQuote
        }
    }

    fn handle_in_double_quote(&mut self, ch: char) -> Result<ParseState, String> {
        match ch {
            '"' => Ok(ParseState::Normal),
            '\\' => {
                // Only escape certain characters within double quotes.
                if let Some(&next_ch) = self.chars.peek() {
                    match next_ch {
                        '\\' | '"' | '$' | '\n' => {
                            self.current.push(self.chars.next().unwrap());
                        }
                        _ => {
                            self.current.push('\\');
                        }
                    }
                    Ok(ParseState::InDoubleQuote)
                } else {
                    Err("Trailing backslash in double quotes".into())
                }
            }
            _ => {
                self.current.push(ch);
                Ok(ParseState::InDoubleQuote)
            }
        }
    }

    fn handle_redirections(&self) -> ParseOutput {
        let mut iter = self.args.iter();
        let mut args = Vec::new();
        let mut stdout_target = None;
        let mut stderr_target = None;
        let mut append = false;
        while let Some(val) = iter.next() {
            match val.as_str() {
                ">" | "1>" | ">>" | "1>>" => {
                    if val.contains(">>") {
                        append = true;
                    }
                    if let Some(file) = iter.next() {
                        stdout_target = Some((file.clone(), append));
                    } else {
                        eprintln!("Error: No file specified for redirection");
                        break;
                    }
                }
                "2>" => {
                    if let Some(file) = iter.next() {
                        stderr_target = Some(file.clone());
                    } else {
                        eprintln!("Error: No file specified for redirection");
                        break;
                    }
                }
                _ => args.push(val.clone()),
            }
        }
        ParseOutput {
            args,
            out_target: stdout_target,
            err_target: stderr_target,
        }
    }
}
