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
        macro_rules! read_digraph {
            ($short:ident, $long:ident) => {
                if self.scanner.eat('=') {
                    TokenKind::$long
                } else {
                    TokenKind::$short
                }
            };
        }

        self.scanner.eat_while(is_char_whitespace);

        let Some(char) = self.scanner.bump() else {
            return Some(TokenKind::Eof);
        };

        let kind = match char {
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            ';' => TokenKind::Semi,
            '/' => {
                if self.scanner.eat('/') {
                    self.scanner.eat_while(is_char_inline);
                    return None;
                }

                TokenKind::Slash
            }
            '*' => TokenKind::Star,
            '!' => read_digraph!(Bang, BangEquals),
            '=' => read_digraph!(Equals, EqualsEquals),
            '>' => read_digraph!(Greater, GreaterEquals),
            '<' => read_digraph!(Less, LessEquals),
            _ => {
                eprintln!("Unexpected character {char:?}.");
                return None;
            }
        };

        Some(kind)
    }
}

/// Returns [`true`] if a [`char`] is not a line feed.
const fn is_char_inline(char: char) -> bool {
    char != '\n'
}

/// Returns [`true`] if a [`char`] is whitespace.
const fn is_char_whitespace(char: char) -> bool {
    matches!(char, '\t' | '\n' | '\r' | ' ')
}
