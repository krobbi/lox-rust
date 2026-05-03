use crate::{
    ast::{Expr, ExprKind, UnOp},
    diagnostics::Diag,
    spans::{BytePos, Span},
    symbols::Symbol,
    tokens::{TokenKind, TokenType},
};

use super::Parser;

impl Parser<'_, '_, '_> {
    /// Parses and returns an [`Expr`].
    pub fn parse_expr(&mut self) -> Expr {
        self.parse_expr_unary()
    }

    /// Parses and returns a unary [`Expr`].
    fn parse_expr_unary(&mut self) -> Expr {
        let start_pos = self.start_pos();

        let op = match self.peek().token_type() {
            TokenType::Minus => UnOp::Minus,
            TokenType::Bang => UnOp::Not,
            _ => return self.parse_expr_call(),
        };

        self.bump();
        let rhs = self.parse_expr_unary();
        self.make_expr(ExprKind::Unary(op, Box::new(rhs)), start_pos)
    }

    /// Parses and returns a call [`Expr`].
    fn parse_expr_call(&mut self) -> Expr {
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
}
