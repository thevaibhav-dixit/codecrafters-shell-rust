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
    ) -> std::io::Result<()> {
        let output = std::process::Command::new(
            self.get_path()
                .file_name()
                .expect("should return file name"),
        )
        .args(&args[1..])
        .output()?;

        out_writer.write_all(&output.stdout)?;
        err_writer.write_all(&output.stderr)
    }
}
