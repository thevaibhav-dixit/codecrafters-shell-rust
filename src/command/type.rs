pub struct Type;

impl super::Runnable for Type {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        let args = &args[1..];

        if let Some(arg) = args.first() {
            match arg.parse::<super::Command>() {
                Ok(super::Command::Builtin(_)) => {
                    writeln!(out_writer, "{} is a shell builtin", arg)
                        .expect("Should pass generally ");
                }
                Ok(super::Command::Binary(path)) => {
                    writeln!(out_writer, "{} is {}", arg, path.get_command())
                        .expect("Should pass generally");
                }
                Ok(super::Command::Unknown) => {
                    writeln!(out_writer, "{}: not found", arg).expect("Should pass generally");
                }
                Err(_) => {
                    writeln!(out_writer, "{} is not a valid command", arg)
                        .expect("Should pass generally");
                }
            }
        } else {
            writeln!(err_writer, "type: not enough arguments").expect("Should pass generally");
        }
    }
}
