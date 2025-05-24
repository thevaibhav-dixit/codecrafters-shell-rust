use std::io::Write;
use std::process::{Command, Stdio};

pub struct Binary(std::path::PathBuf);

impl Binary {
    pub fn new(command: std::path::PathBuf) -> Self {
        Self(command)
    }

    pub fn get_path(&self) -> &std::path::PathBuf {
        &self.0
    }

    fn is_long_running_command(&self, args: &[String]) -> bool {
        let command_name = self
            .get_path()
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        let long_running_commands = [
            "tail", "watch", "top", "htop", "less", "more", "vi", "vim", "nano",
        ];

        long_running_commands.contains(&command_name)
            || args.contains(&"-f".to_string())
            || args.contains(&"--follow".to_string())
    }
}

impl super::Runnable for Binary {
    fn run(
        &self,
        args: Vec<String>,
        input: Option<&mut dyn std::io::Read>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));

        let mut piped_input = String::new();
        if let Some(input) = input {
            input.read_to_string(&mut piped_input)?;
        }

        // For long-running commands, inherit stdio to show output directly
        if self.is_long_running_command(&args) {
            let mut cmd = Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            );

            cmd.args(&args[1..]);

            // Handle piped input
            if !piped_input.is_empty() {
                cmd.stdin(Stdio::piped());
            } else {
                cmd.stdin(Stdio::inherit()); // Allow interactive input if needed
            }

            // Inherit stdout and stderr so output goes directly to terminal
            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

            let mut child = cmd.spawn()?;

            // Send piped input if we have it
            if !piped_input.is_empty() {
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(piped_input.as_bytes())?;
                    drop(stdin);
                }
            }

            // Wait for the process to complete (or be interrupted)
            let _ = child.wait()?;

            return Ok(());
        }

        // Handle regular commands (your existing logic)
        if piped_input.is_empty() {
            let output = Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            )
            .args(&args[1..])
            .stdin(Stdio::null()) // Explicitly close stdin
            .output()?;

            out_writer.write_all(&output.stdout)?;
            err_writer.write_all(&output.stderr)?;
        } else {
            let mut child = Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            )
            .args(&args[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(piped_input.as_bytes())?;
            }

            let output = child.wait_with_output()?;
            out_writer.write_all(&output.stdout)?;
            err_writer.write_all(&output.stderr)?;
        }

        Ok(())
    }
}
