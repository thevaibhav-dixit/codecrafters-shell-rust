pub struct Pwd;

impl<W: std::io::Write> super::Runnable<W> for Pwd {
    fn run(
        &self,
        _args: Vec<String>,
        _input: Option<&mut dyn std::io::Read>,
        out_writer: &mut W,
        _err_writer: &mut W,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push("pwd".to_string());
        let current_dir = std::env::current_dir()?;
        writeln!(out_writer, "{}", current_dir.display())
    }
}
