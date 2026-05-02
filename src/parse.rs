use crate::{
    ast::{Ast, Expr, ExprKind, Ident},
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
        }
    }

    /// Parses and returns an [`Ast`].
    fn parse_ast(&mut self) -> Ast {
        let expr = self.parse_expr();
        Ast(expr)
    }

    /// Parses and returns an [`Expr`].
    fn parse_expr(&mut self) -> Expr {
        self.parse_expr_primary()
    }

    /// Parses and returns a primary [`Expr`].
    fn parse_expr_primary(&mut self) -> Expr {
        let start_pos = self.start_pos();

        let kind = match self.peek().kind() {
            TokenKind::Literal(literal) => {
                self.bump();
                ExprKind::Literal(literal)
            }
            TokenKind::Ident(symbol) => {
                self.bump();
                ExprKind::Variable(symbol)
            }
            TokenKind::OpenParen => {
                self.bump();
                let expr = self.parse_expr();
                self.expect(TokenType::CloseParen);
                ExprKind::Paren(Box::new(expr))
            }
            TokenKind::Super => {
                self.bump();
                self.expect(TokenType::Dot);
                let ident = self.parse_ident();
                ExprKind::Super(ident)
            }
            TokenKind::This => {
                self.bump();
                ExprKind::This
            }
            kind => {
                let span = self.peek().span();
                return self.error_expr(Diag::ExpectedExpr(kind), span);
            }
        };

        self.make_expr(kind, start_pos)
    }

    /// Parses and returns an [`Ident`].
    fn parse_ident(&mut self) -> Ident {
        let span = self.peek().span();

        let symbol = if let TokenKind::Ident(symbol) = self.next_token.kind() {
            self.bump();
            symbol
        } else {
            let diag = Diag::UnexpectedToken(TokenType::Ident, self.next_token.kind());

            if self.next_token.is_keyword() {
                self.bump();
                self.report_recovered(diag, span);
            } else {
                self.report(diag, span);
            }

            Symbol::ERROR
        };

        Ident { symbol, span }
    }

    /// Returns the next [`Token`]'s start [`BytePos`].
    const fn start_pos(&self) -> BytePos {
        self.peek().span().start()
    }

    /// Returns a new [`Span`] from a start [`BytePos`] to the previous
    /// [`Token`]'s end [`BytePos`].
    fn span_from(&self, start_pos: BytePos) -> Span {
        Span::new(start_pos, self.prev_token_end_pos)
    }

    /// Returns a new [`Expr`] from an [`ExprKind`] and a start [`BytePos`].
    fn make_expr(&self, kind: ExprKind, start_pos: BytePos) -> Expr {
        let span = self.span_from(start_pos);
        Expr { kind, span }
    }

    /// Reports a [`Diag`] at a [`Span`] and returns a new synthetic [`Expr`]
    /// for error recovery.
    fn error_expr(&mut self, diag: Diag, span: Span) -> Expr {
        self.report(diag, span);

        Expr {
            kind: ExprKind::Variable(Symbol::ERROR),
            span,
        }
    }

    /// Reports a [`Diag`] at a [`Span`].
    fn report(&mut self, diag: Diag, span: Span) {
        self.report_recovered(diag, span);
        self.bump(); // TODO: Replace with panic mode.
    }

    /// Reports a recovered [`Diag`] at a [`Span`].
    fn report_recovered(&mut self, diag: Diag, span: Span) {
        self.lexer.log_mut().report(diag, span);
    }

    /// Returns the next [`Token`] without consuming it.
    const fn peek(&self) -> &Token {
        &self.next_token
    }

    /// Consumes the next [`Token`].
    fn bump(&mut self) {
        self.prev_token_end_pos = self.peek().span().end();
        self.next_token = self.lexer.next_token();
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns [`true`] if a [`Token`] was consumed.
    fn eat(&mut self, token_type: TokenType) -> bool {
        let is_match = self.peek().token_type() == token_type;

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
                Diag::UnexpectedToken(token_type, self.peek().kind()),
                self.peek().span(),
            );
        }
    }
}
