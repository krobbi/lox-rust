mod scan;

use crate::{
    diagnostics::Diag,
    log::Log,
    spans::{BytePos, Span},
    symbols::SymbolTable,
    tokens::{Literal, Token, TokenKind},
};

use self::scan::Scanner;

/// A structure which reads a stream of [`Token`]s from source code.
pub struct Lexer<'src, 'sym, 'log> {
    /// The [`Scanner`].
    scanner: Scanner<'src>,

    /// The [`SymbolTable`].
    symbols: &'sym mut SymbolTable,

    /// The [`Log`].
    log: &'log mut Log,

    /// The next [`Token`]'s start [`BytePos`].
    pos: BytePos,
}

impl<'src, 'sym, 'log> Lexer<'src, 'sym, 'log> {
    /// Creates a new `Lexer` from source code, a [`SymbolTable`], and a
    /// [`Log`].
    pub fn new(source: &'src str, symbols: &'sym mut SymbolTable, log: &'log mut Log) -> Self {
        Self {
            scanner: Scanner::new(source),
            symbols,
            log,
            pos: BytePos::new(),
        }
    }

    /// Returns the next [`Token`].
    pub fn next_token(&mut self) -> Token {
        loop {
            self.scanner.begin_lexeme();
            let kind = self.next_token_kind();

            let start = self.pos;
            let end = start + self.scanner.lexeme_length();
            self.pos = end;

            if let Some(kind) = kind {
                let span = Span::new(start, end);
                break Token::new(kind, span);
            }
        }
    }

    /// Returns the next [`TokenKind`]. This function returns [`None`] if
    /// whitespace, a comment, or an error was encountered.
    fn next_token_kind(&mut self) -> Option<TokenKind> {
        let Some(char) = self.scanner.bump() else {
            return Some(TokenKind::Eof);
        };

        let kind = match char {
            c if is_char_whitespace(c) => {
                self.scanner.eat_while(is_char_whitespace);
                return None;
            }
            c if is_char_word_start(c) => self.next_word(),
            c if is_char_digit(c) => self.next_number(),
            '"' => self.next_string(),
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            '.' => {
                if self.is_digit_next() {
                    let start = self.pos;
                    let end = start;
                    let span = Span::new(start, end);
                    self.log.report(Diag::LeadingDecimal, span);

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
            '!' => self.next_digraph(TokenKind::Bang, TokenKind::BangEquals),
            '=' => self.next_digraph(TokenKind::Equals, TokenKind::EqualsEquals),
            '>' => self.next_digraph(TokenKind::Greater, TokenKind::GreaterEquals),
            '<' => self.next_digraph(TokenKind::Less, TokenKind::LessEquals),
            _ => {
                let start = self.pos;
                let end = start + self.scanner.lexeme_length();
                let span = Span::new(start, end);
                self.log.report(Diag::UnexpectedChar(char), span);

                return None;
            }
        };

        Some(kind)
    }

    /// Returns the next keyword or identifier [`TokenKind`] after consuming its
    /// first letter [`char`].
    fn next_word(&mut self) -> TokenKind {
        self.scanner.eat_while(is_char_word_continue);

        match self.scanner.lexeme() {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::Literal(Literal::Bool(false)),
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Literal(Literal::Nil),
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::Literal(Literal::Bool(true)),
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            name => TokenKind::Ident(self.symbols.intern(name)),
        }
    }

    /// Returns the next number literal [`TokenKind`] after consuming its first
    /// digit [`char`].
    fn next_number(&mut self) -> TokenKind {
        self.scanner.eat_while(is_char_digit);

        // NOTE: Lox specifies that a trailing decimal point should be lexed as
        // a separate dot token. This is technically a breaking change, but it
        // allows the lexer to use single-character lookahead. Additionally, a
        // number of the left-hand side of a dot operator is always a type
        // error.
        if self.scanner.eat('.') {
            if !self.is_digit_next() {
                let start = self.pos + (self.scanner.lexeme_length() - 1);
                let end = start + 1;
                let span = Span::new(start, end);
                self.log.report(Diag::TrailingDecimal, span);
            }

            self.scanner.eat_while(is_char_digit);
        }

        self.number_from_lexeme()
    }

    /// Returns the next string literal [`TokenKind`] after consuming its
    /// opening quote [`char`].
    fn next_string(&mut self) -> TokenKind {
        self.scanner.eat_while(is_char_in_string);

        #[expect(clippy::string_slice, reason = "first code point is a 1-byte quote")]
        let value = &self.scanner.lexeme()[1..];

        if !self.scanner.eat('"') {
            let start = self.pos + 1;
            let end = self.pos + self.scanner.lexeme_length();
            let span = Span::new(start, end);
            self.log.report(Diag::UnterminatedString, span);
        }

        TokenKind::Literal(Literal::String(self.symbols.intern(value)))
    }

    /// Returns the next short or long [`TokenKind`] depending on whether an
    /// equals sign [`char`] is consumed.
    fn next_digraph(&mut self, short: TokenKind, long: TokenKind) -> TokenKind {
        if self.scanner.eat('=') { long } else { short }
    }

    /// Returns [`true`] if the next [`char`] is a digit.
    fn is_digit_next(&self) -> bool {
        matches!(self.scanner.peek(), Some(c) if is_char_digit(c))
    }

    /// Returns a new number [`TokenKind`] from the current lexeme.
    fn number_from_lexeme(&self) -> TokenKind {
        // At this point, the current lexeme must follow this grammar:
        // https://doc.rust-lang.org/std/primitive.f64.html#grammar
        let value = self.scanner.lexeme();
        let value = value.parse().expect("lexeme should be a valid float");
        TokenKind::Literal(Literal::Number(value))
    }
}

/// Returns [`true`] if a [`char`] is whitespace.
const fn is_char_whitespace(char: char) -> bool {
    matches!(char, '\t' | '\n' | '\r' | ' ')
}

/// Returns [`true`] if a [`char`] is not a line feed.
const fn is_char_inline(char: char) -> bool {
    char != '\n'
}

/// Returns [`true`] if a [`char`] is a keyword or identifier start.
const fn is_char_word_start(char: char) -> bool {
    char.is_ascii_alphabetic() || char == '_'
}

/// Returns [`true`] if a [`char`] is a keyword or identifier continuation.
const fn is_char_word_continue(char: char) -> bool {
    char.is_ascii_alphanumeric() || char == '_'
}

/// Returns [`true`] if a [`char`] is a digit.
const fn is_char_digit(char: char) -> bool {
    char.is_ascii_digit()
}

/// Returns [`true`] if a [`char`] is not a quote.
const fn is_char_in_string(char: char) -> bool {
    char != '"'
}
