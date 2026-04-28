mod lex;
mod tokens;

use std::{env, fs, path::Path, process::ExitCode};

use crate::{lex::Lexer, tokens::TokenType};

/// Runs Lox and returns an [`ExitCode`].
fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);

    let result = match args.len() {
        0 => todo!("REPL mode"),
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

/// Interprets a source file from its [`Path`]. This function returns [`Err`] if
/// the source file could not be read.
fn interpret_file(path: &Path) -> Result<(), ()> {
    let Ok(source) = fs::read_to_string(path) else {
        eprintln!("Could not read file {:?}", path.to_string_lossy());
        return Err(());
    };

    interpret_source(&source);
    Ok(())
}

/// Interprets source code.
fn interpret_source(source: &str) {
    let mut lexer = Lexer::new(source);

    loop {
        let token = lexer.next_token();
        println!("{token}");

        if token.token_type() == TokenType::Eof {
            break;
        }
    }
}
