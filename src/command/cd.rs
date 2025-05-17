pub struct Cd;

impl Cd {
    fn get_home_dir(&self) -> std::path::PathBuf {
        std::env::var("HOME")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::path::PathBuf::from("/home"))
    }
}

impl super::Runnable for Cd {
    fn run(
        &self,
        args: Vec<String>,
        _out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        let path = if let Some(path) = args.get(1) {
            if path == "~" {
                self.get_home_dir()
            } else {
                std::path::PathBuf::from(path)
            }
        } else {
            self.get_home_dir()
        };

        if std::env::set_current_dir(&path).is_err() {
            writeln!(
                err_writer,
                "cd: {}: No such file or directory",
                path.display(),
            )?
        }
        Ok(())
    }
}
