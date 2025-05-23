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

        for (i, line) in history.iter().enumerate() {
            write!(out_writer, "    {}  {}\n", i + 1, line)?;
        }

        Ok(())
    }
}
