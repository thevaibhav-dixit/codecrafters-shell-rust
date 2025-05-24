pub struct Type;

impl<W: std::io::Write> super::Runnable<W> for Type {
    fn run(
        &self,
        args: Vec<String>,
        _input: Option<&mut dyn std::io::Read>,
        out_writer: &mut W,
        err_writer: &mut W,
        history: &mut Vec<String>,
    ) -> std::io::Result<()> {
        history.push(args.join(" "));
        let args = &args[1..];

        if let Some(arg) = args.first() {
            match arg.parse::<super::Command>() {
                Ok(super::Command::Builtin(_)) => {
                    out_writer.write_all(format!("{} is a shell builtin\n", arg).as_bytes())
                }
                Ok(super::Command::Binary(path)) => out_writer
                    .write_all(format!("{} is {}\n", arg, path.get_path().display()).as_bytes()),
                Ok(super::Command::Unknown(_)) => {
                    out_writer.write_all(format!("{}: not found\n", arg).as_bytes())
                }
                Err(_) => {
                    err_writer.write_all(format!("{} is not a valid command\n", arg).as_bytes())
                }
            }
        } else {
            err_writer.write_all(b"type: not enough arguments")
        }
    }
}
