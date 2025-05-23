pub struct Exit;

impl super::Runnable for Exit {
    fn run(
        &self,
        args: Vec<String>,
        _out_writer: &mut dyn std::io::Write,
        _err_writer: &mut dyn std::io::Write,
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
