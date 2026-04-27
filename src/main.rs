mod scan;

use std::{env, fs, path::Path, process::ExitCode};

use crate::scan::Scanner;

/// Runs Lox and returns an [`ExitCode`].
fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);

    if args.len() != 1 {
        eprintln!("Usage: lox <path>");
        return ExitCode::FAILURE;
    }

    let path = args.next().expect("argument count should be checked");
    let path = Path::new(&path);

    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("Could not read file {:?}.", path.to_string_lossy());
        return ExitCode::FAILURE;
    };

    interpret_source(&source);
    ExitCode::SUCCESS
}

/// Interprets source code.
fn interpret_source(source: &str) {
    let mut scanner = Scanner::new(source);

    while let Some(char) = scanner.bump() {
        println!("{char:?}");
    }
}
