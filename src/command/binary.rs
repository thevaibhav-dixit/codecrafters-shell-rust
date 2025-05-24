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

        let long_running_commands = ["watch", "top", "htop", "less", "more", "vi", "vim", "nano"];

        long_running_commands.contains(&command_name)
            || args.iter().any(|arg| arg == "-f" || arg == "--follow")
    }
}

impl<W: std::io::Write> super::Runnable<W> for Binary {
    fn run(
        &self,
        args: Vec<String>,
        input: Option<&mut dyn std::io::Read>,
        out_writer: &mut W,
        err_writer: &mut W,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));

        let mut piped_input = String::new();
        if let Some(input) = input {
            input.read_to_string(&mut piped_input)?;
        }

        if self.is_long_running_command(&args) {
            let mut cmd = Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            );

            cmd.args(&args[1..]);

            if !piped_input.is_empty() {
                cmd.stdin(Stdio::piped());
            } else {
                cmd.stdin(Stdio::inherit());
            }

            cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

            let mut child = cmd.spawn()?;

            let _ = child.wait()?;

            return Ok(());
        }

        if piped_input.is_empty() {
            let output = Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            )
            .args(&args[1..])
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
