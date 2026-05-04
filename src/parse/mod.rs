mod exprs;
mod stmts;

use crate::{
    ast::{Ast, Ident, Stmt},
    diagnostics::Diag,
    lex::Lexer,
    log::Log,
    spans::{BytePos, Span},
    symbols::{Symbol, SymbolTable},
    tokens::{Token, TokenKind, TokenType},
};

/// Parses and returns an [`Ast`] from source code, a [`SymbolTable`], and a
/// [`Log`].
pub fn parse_source(source: &str, symbols: &mut SymbolTable, log: &mut Log) -> Ast {
    let mut parser = Parser::new(source, symbols, log);
    parser.parse_ast()
}

/// A structure which parses an [`Ast`] from source code.
struct Parser<'src, 'sym, 'log> {
    /// The [`Lexer`].
    lexer: Lexer<'src, 'sym, 'log>,

    /// The next [`Token`].
    next_token: Token,

    /// The previous [`Token`]'s end [`BytePos`].
    prev_token_end_pos: BytePos,

    /// Whether the `Parser` is in panic mode.
    is_panicking: bool,
}

impl<'src, 'sym, 'log> Parser<'src, 'sym, 'log> {
    /// Creates a new `Paser` from source code, a [`SymbolTable`], and a
    /// [`Log`].
    fn new(source: &'src str, symbols: &'sym mut SymbolTable, log: &'log mut Log) -> Self {
        let mut lexer = Lexer::new(source, symbols, log);
        let next_token = lexer.next_token();

        Self {
            lexer,
            next_token,
            prev_token_end_pos: BytePos::new(),
            is_panicking: false,
        }
    }

    /// Parses and returns an [`Ast`].
    fn parse_ast(&mut self) -> Ast {
        let mut decls = Vec::new();

        while self.peek() != TokenType::Eof {
            let decl = self.parse_decl();
            decls.push(decl);
        }

        Ast(decls.into_boxed_slice())
    }

    /// Parses and returns a declaration [`Stmt`].
    fn parse_decl(&mut self) -> Stmt {
        let stmt = self.parse_stmt();
        self.synchronize();
        stmt
    }

    /// Parses and returns an [`Ident`].
    fn parse_ident(&mut self) -> Ident {
        let start_pos = self.start_pos();

        let symbol = if let TokenKind::Ident(symbol) = self.next_token.kind() {
            self.bump();
            symbol
        } else {
            let diag = Diag::UnexpectedToken(TokenType::Ident, self.next_token.kind());
            let span = self.next_token.span();

            if self.next_token.is_keyword() {
                self.bump();
                self.report_recovered(diag, span);
            } else {
                self.report(diag, span);
            }

            Symbol::ERROR
        };

        Ident {
            symbol,
            span: self.span_from(start_pos),
        }
    }

    /// Returns the next [`Token`]'s start [`BytePos`].
    const fn start_pos(&self) -> BytePos {
        self.next_token.span().start()
    }

    /// Returns a new [`Span`] from a start [`BytePos`] to the previous
    /// [`Token`]'s end [`BytePos`].
    fn span_from(&self, start_pos: BytePos) -> Span {
        Span::new(start_pos, self.prev_token_end_pos.max(start_pos))
    }

    /// Returns the next [`Token`]'s [`TokenType`].
    const fn peek(&self) -> TokenType {
        self.next_token.token_type()
    }

    /// Consumes the next [`Token`].
    fn bump(&mut self) {
        debug_assert_ne!(
            self.peek(),
            TokenType::Eof,
            "parser consumed end of file token"
        );

        self.prev_token_end_pos = self.next_token.span().end();
        self.next_token = self.lexer.next_token();
    }

    /// Consumes the next [`Token`], which should always match an expected
    /// [`TokenType`]. This function must only be used where it can *never* fail
    /// to user syntax error.
    fn bump_assert(&mut self, token_type: TokenType) {
        debug_assert_eq!(
            self.peek(),
            token_type,
            "parser did not consume expected token type"
        );

        self.bump();
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns [`true`] if a [`Token`] was consumed.
    fn eat(&mut self, token_type: TokenType) -> bool {
        let is_match = self.peek() == token_type;

        if is_match {
            self.bump();
        }

        is_match
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function reports a [`Diag`] if no [`Token`] was consumed.
    fn expect(&mut self, token_type: TokenType) {
        if !self.eat(token_type) {
            self.report(
                Diag::UnexpectedToken(token_type, self.next_token.kind()),
                self.next_token.span(),
            );
        }
    }

    /// Reports a [`Diag`] at a [`Span`].
    fn report(&mut self, diag: Diag, span: Span) {
        self.report_recovered(diag, span);
        self.is_panicking = true;
    }

    /// Reports a recovered [`Diag`] at a [`Span`].
    fn report_recovered(&mut self, diag: Diag, span: Span) {
        if self.is_panicking {
            return;
        }

        self.lexer.log_mut().report(diag, span);
    }

    /// Synchronizes the `Parser` out of panic mode.
    fn synchronize(&mut self) {
        if !self.is_panicking {
            return;
        }

        while self.peek() != TokenType::Eof {
            self.bump();

            match self.peek() {
                TokenType::Semi => {
                    self.bump();
                    break;
                }
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => break,
                _ => (),
            }
        }

        self.is_panicking = false;
    }
}
