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
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        match std::process::Command::new(&args[0])
            .args(&args[1..])
            .output()
        {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    out_writer.write_all(&output.stdout).unwrap();
                }
                if !output.stderr.is_empty() {
                    err_writer.write_all(&output.stderr).unwrap();
                }
            }
            Err(e) => {
                writeln!(err_writer, "Error executing command: {}", e).unwrap();
            }
        }
    }
}
