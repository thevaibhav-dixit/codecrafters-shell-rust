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
};

struct ShellCompleter {
    commands: HashSet<String>,
    highlighter: MatchingBracketHighlighter,
}

impl ShellCompleter {
    fn new() -> Self {
        let mut commands = HashSet::new();

        commands.insert("exit".to_string());

        if let Some(paths) = std::env::var_os("PATH") {
            for path in std::env::split_paths(&paths) {
                if let Ok(entries) = std::fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                            if path.is_file() && is_executable(&path) {
                                commands.insert(file_name.to_string());
                            }
                        }
                    }
                }
            }
        }

        ShellCompleter {
            commands,
            highlighter: MatchingBracketHighlighter::new(),
        }
    }
}

fn is_executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .map(|m| m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
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

            let mut matches: Vec<_> = self
                .commands
                .iter()
                .filter(|cmd| cmd.starts_with(prefix))
                .collect();

            matches.sort();

            let candidates: Vec<Pair> = if matches.len() == 1 {
                vec![Pair {
                    display: matches[0].clone(),
                    replacement: format!("{} ", matches[0]),
                }]
            } else {
                matches
                    .into_iter()
                    .map(|cmd| Pair {
                        display: cmd.clone(),
                        replacement: cmd.to_string(),
                    })
                    .collect()
            };

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

pub struct ReadLine {
    rl: Editor<ShellCompleter, rustyline::history::DefaultHistory>,
}

impl ReadLine {
    pub fn new() -> Self {
        let config = Config::builder()
            .completion_type(CompletionType::List)
            .build();

        let mut rl = Editor::with_config(config).unwrap();
        rl.set_helper(Some(ShellCompleter::new()));

        ReadLine { rl }
    }

    pub fn readline(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        self.rl.readline(prompt)
    }
}
