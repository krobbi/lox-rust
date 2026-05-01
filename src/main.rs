mod ast;
mod diagnostics;
mod lex;
mod log;
mod parse;
mod render;
mod spans;
mod symbols;
mod tokens;

use std::{
    env, fs,
    io::{self, Write as _},
    path::Path,
    process::ExitCode,
};

use crate::{log::Log, render::RenderContext, symbols::SymbolTable};

/// Runs Lox and returns an [`ExitCode`].
fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);

    let result = match args.len() {
        0 => {
            run_repl();
            Ok(())
        }
        1 => {
            let path = args.next().expect("argument count should be checked");
            let path = Path::new(&path);
            interpret_file(path)
        }
        _ => {
            eprintln!("Usage: lox [path]");
            Err(())
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}

/// Runs Lox in REPL mode.
fn run_repl() {
    let mut source = String::new();

    loop {
        print!("> ");
        io::stdout()
            .flush()
            .expect("flushing stdout should not fail");

        source.clear();

        if let Err(error) = io::stdin().read_line(&mut source) {
            eprintln!("Could not read line: {error}");
            continue;
        }

        if source.is_empty() {
            println!();
            break;
        }

        interpret_source(&source);
    }
}

/// Interprets a source file from its [`Path`]. This function returns [`Err`] if
/// the source file could not be read.
fn interpret_file(path: &Path) -> Result<(), ()> {
    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("Could not read file {:?}.", path.to_string_lossy());
        return Err(());
    };

    interpret_source(&source);
    Ok(())
}

/// Interprets source code.
fn interpret_source(source: &str) {
    let mut symbols = SymbolTable::new();
    let mut log = Log::new();
    let ast = parse::parse_source(source, &mut symbols, &mut log);
    println!("{ast:#?}");

    let ctx = RenderContext::new(source, &symbols);
    log.flush(ctx);
}
