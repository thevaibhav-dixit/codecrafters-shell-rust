pub struct Echo;

impl super::Runnable for Echo {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        _err_writer: &mut dyn std::io::Write,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));
        let args = args[1..].join(" ") + "\n";
        out_writer.write_all(args.as_bytes())
    }
}
