use std::{env, path::Path, process::ExitCode};

/// Runs Lox and returns an [`ExitCode`].
fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);

    if args.len() == 1 {
        let path = args.next().expect("argument count should be checked");
        let path = Path::new(&path);
        println!("Path: {:?}", path.to_string_lossy());
        ExitCode::SUCCESS
    } else {
        eprintln!("Usage: lox <path>");
        ExitCode::FAILURE
    }
}
