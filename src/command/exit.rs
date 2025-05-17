pub struct Exit;

impl super::Runnable for Exit {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        let args = &args[1..];
        if args.is_empty() {
            std::process::exit(0);
        } else {
            let code = args[0].parse::<i32>().unwrap_or(1);
            std::process::exit(code);
        }
    }
}
