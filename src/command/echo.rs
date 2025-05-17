pub struct Echo;

impl super::Runnable for Echo {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        let args = &args[1..];

        writeln!(out_writer, "{}", args.join(" "));
    }
}
