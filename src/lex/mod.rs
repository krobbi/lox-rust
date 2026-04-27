mod scan;

use crate::tokens::{Token, TokenKind};

use self::scan::Scanner;

/// A structure which reads a stream of [`Token`]s from source code.
pub struct Lexer<'src> {
    /// The [`Scanner`].
    scanner: Scanner<'src>,
}

impl<'src> Lexer<'src> {
    /// Creates a new `Lexer` from source code.
    pub fn new(source: &'src str) -> Self {
        Self {
            scanner: Scanner::new(source),
        }
    }

    /// Returns the next [`Token`].
    pub fn next_token(&mut self) -> Token {
        loop {
            if let Some(kind) = self.next_token_kind() {
                break Token::new(kind);
            }
        }
    }

    /// Returns the next [`TokenKind`]. This function returns [`None`] if a
    /// comment or error was encountered.
    fn next_token_kind(&mut self) -> Option<TokenKind> {
        self.scanner.eat_while(is_char_whitespace);

        let Some(char) = self.scanner.bump() else {
            return Some(TokenKind::Eof);
        };

        #[expect(
            unused_variables,
            clippy::match_single_binding,
            reason = "more token kinds will be added later"
        )]
        let kind = match char {
            _ => {
                eprintln!("Unexpected character {char:?}.");
                return None;
            }
        };

        #[expect(unreachable_code, reason = "more token kinds will be added later")]
        Some(kind)
    }
}

/// Returns [`true`] if a [`char`] is whitespace.
const fn is_char_whitespace(char: char) -> bool {
    matches!(char, '\t' | '\n' | '\r' | ' ')
}
