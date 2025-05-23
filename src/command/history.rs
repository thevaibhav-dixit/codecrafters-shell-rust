pub struct History;

impl super::Runnable for History {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        _err_writer: &mut dyn std::io::Write,
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
