mod scan;

use crate::tokens::{Literal, Token, TokenKind};

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
        self.scanner.begin_lexeme();

        let Some(char) = self.scanner.bump() else {
            return Some(TokenKind::Eof);
        };

        let kind = match char {
            c if is_char_digit(c) => self.next_number(),
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            '.' => {
                // NOTE: A number on the right-hand side of a dot operator is
                // always a syntax error, so this is a valid recovery.
                if self.is_digit_next() {
                    eprintln!("Number has a leading decimal point.");
                    self.scanner.eat_while(is_char_digit);
                    self.number_from_lexeme()
                } else {
                    TokenKind::Dot
                }
            }
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

    /// Returns the next number [`TokenKind`] after consuming its first
    /// [`char`].
    fn next_number(&mut self) -> TokenKind {
        self.scanner.eat_while(is_char_digit);

        // NOTE: Lox specifies that a trailing decimal point should be lexed as
        // a separate dot token. This is technically a breaking change, but it
        // allows the lexer to use single-character lookahead. Additionally, a
        // number of the left-hand side of a dot operator is always a type
        // error.
        if self.scanner.eat('.') {
            if !self.is_digit_next() {
                eprintln!("Number has a trailing decimal point.");
            }

            self.scanner.eat_while(is_char_digit);
        }

        self.number_from_lexeme()
    }

    /// Returns [`true`] if the next [`char`] is a digit.
    fn is_digit_next(&self) -> bool {
        matches!(self.scanner.peek(), Some(c) if is_char_digit(c))
    }

    /// Returns a new number [`TokenKind`] from the current lexeme.
    fn number_from_lexeme(&self) -> TokenKind {
        // NOTE: The lexeme must follow Rust's grammar for parsing a float:
        // https://doc.rust-lang.org/std/primitive.f64.html#grammar
        let value = self.scanner.lexeme();
        let value = value.parse().expect("lexeme should be a valid float");
        TokenKind::Literal(Literal::Number(value))
    }
}

/// Returns [`true`] if a [`char`] is a digit.
const fn is_char_digit(char: char) -> bool {
    char.is_ascii_digit()
}

/// Returns [`true`] if a [`char`] is not a line feed.
const fn is_char_inline(char: char) -> bool {
    char != '\n'
}

/// Returns [`true`] if a [`char`] is whitespace.
const fn is_char_whitespace(char: char) -> bool {
    // NOTE: `char::is_ascii_whitespace` includes form feed, which is not
    // specified as whitespace in Lox.
    matches!(char, '\t' | '\n' | '\r' | ' ')
}
