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
        let mut start = 0;
        let end = history.len();

        if args.len() > 0 {
            if let Ok(num) = args[0].parse::<usize>() {
                start = end - num;
            } else {
                writeln!(out_writer, "Invalid argument: {}", args[0])?;
                return Ok(());
            }
        }

        for (i, line) in history[start..].iter().enumerate() {
            write!(out_writer, "    {}  {}\n", start + i + 1, line)?;
        }

        Ok(())
    }
}
