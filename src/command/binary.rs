use std::io::Write;

pub struct Binary(std::path::PathBuf);

impl Binary {
    pub fn new(command: std::path::PathBuf) -> Self {
        Self(command)
    }

    pub fn get_path(&self) -> &std::path::PathBuf {
        &self.0
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

        if piped_input.is_empty() {
            let child = std::process::Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            )
            .args(&args[1..])
            .spawn()?;

            let output = child.wait_with_output()?;
            out_writer.write_all(&output.stdout)?;
            err_writer.write_all(&output.stderr)?;
        } else {
            let mut child = std::process::Command::new(
                self.get_path()
                    .file_name()
                    .expect("should return file name"),
            )
            .args(&args[1..])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
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
