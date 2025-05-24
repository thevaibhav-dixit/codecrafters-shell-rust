pub struct History;

impl<W: std::io::Write> super::Runnable<W> for History {
    fn run(
        &self,
        args: Vec<String>,
        _input: Option<&mut dyn std::io::Read>,
        out_writer: &mut W,
        _err_writer: &mut W,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));

        let args = &args[1..];
        let start = args
            .get(0)
            .and_then(|s| s.parse::<usize>().ok())
            .map_or(0, |n| history.len().saturating_sub(n));

        for (i, line) in history[start..].iter().enumerate() {
            writeln!(out_writer, "    {}  {}", start + i + 1, line)?;
        }
        Ok(())
    }
}
