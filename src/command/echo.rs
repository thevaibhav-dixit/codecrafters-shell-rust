pub struct Echo;

impl<W: std::io::Write> super::Runnable<W> for Echo {
    fn run(
        &self,
        args: Vec<String>,
        _input: Option<&mut dyn std::io::Read>,
        out_writer: &mut W,
        _err_writer: &mut W,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));
        let args = args[1..].join(" ") + "\n";
        out_writer.write_all(args.as_bytes())
    }
}
