pub struct Pwd;

impl super::Runnable for Pwd {
    fn run(
        &self,
        args: Vec<String>,
        out_writer: &mut dyn std::io::Write,
        err_writer: &mut dyn std::io::Write,
    ) {
        write!(out_writer, "{}", std::env::current_dir().unwrap().display()).unwrap();
    }
}
