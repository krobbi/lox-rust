use std::mem;

use crate::{
    ast::{Ast, Expr, ExprKind},
    diagnostics::Diag,
    lex::Lexer,
    log::Log,
    spans::{BytePos, Span},
    symbols::SymbolTable,
    tokens::{Literal, Token, TokenKind, TokenType},
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
    pos: BytePos,
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
            pos: BytePos::new(),
        }
    }

    /// Parses and returns an [`Ast`].
    fn parse_ast(&mut self) -> Ast {
        let expr = self.parse_expr();
        Ast(expr)
    }

    /// Parses and returns an [`Expr`].
    fn parse_expr(&mut self) -> Expr {
        self.parse_expr_atom()
    }

    /// Parses and returns an atom [`Expr`].
    fn parse_expr_atom(&mut self) -> Expr {
        let token = self.bump();
        let start = token.span().start();

        let kind = match token.kind() {
            TokenKind::Literal(literal) => ExprKind::Literal(literal),
            TokenKind::Ident(symbol) => ExprKind::Variable(symbol),
            TokenKind::OpenParen => {
                let expr = self.parse_expr();
                self.expect(TokenType::CloseParen);
                ExprKind::Paren(Box::new(expr))
            }
            TokenKind::This => ExprKind::This,
            kind => {
                self.report(Diag::ExpectedExpr(kind), token.span());
                error_expr()
            }
        };

        self.create_expr(kind, start)
    }

    /// Creates a new [`Expr`] from an [`ExprKind`] and a start [`BytePos`].
    fn create_expr(&self, kind: ExprKind, start: BytePos) -> Expr {
        Expr {
            kind,
            span: Span::new(start, self.pos),
        }
    }

    /// Reports a [`Diag`] at a [`Span`].
    fn report(&mut self, diag: Diag, span: Span) {
        self.lexer.log_mut().report(diag, span);
    }

    /// Consumes and returns the next [`Token`].
    fn bump(&mut self) -> Token {
        self.pos = self.next_token.span().end();
        mem::replace(&mut self.next_token, self.lexer.next_token())
    }

    /// Consumes the next [`Token`] if it matches an expected [`TokenType`].
    /// This function returns [`true`] if a [`Token`] was consumed.
    fn eat(&mut self, token_type: TokenType) -> bool {
        let is_match = self.next_token.token_type() == token_type;

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
}

/// Returns a new synthetic [`ExprKind`] for error recovery.
const fn error_expr() -> ExprKind {
    ExprKind::Literal(Literal::Nil)
}
