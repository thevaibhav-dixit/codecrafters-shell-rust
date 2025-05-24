pub struct Exit;

impl<W: std::io::Write> super::Runnable<W> for Exit {
    fn run(
        &self,
        args: Vec<String>,
        _input: Option<&mut dyn std::io::Read>,
        _out_writer: &mut W,
        _err_writer: &mut W,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));
        let args = &args[1..];
        let code = if args.is_empty() {
            0
        } else {
            args[0].parse::<i32>().unwrap_or(1)
        };
        std::process::exit(code);
    }
}
