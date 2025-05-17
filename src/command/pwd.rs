pub struct Pwd;

impl super::Runnable for Pwd {
    fn run(
        &self,
        _args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        _err_writer: &mut dyn std::io::Write,
    ) -> std::io::Result<()> {
        let current_dir = std::env::current_dir()?;
        writeln!(out_writer, "{}", current_dir.display())
    }
}
