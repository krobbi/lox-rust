use crate::{
    ast::{Stmt, StmtKind},
    spans::BytePos,
    tokens::TokenType,
};

use super::Parser;

impl Parser<'_, '_, '_> {
    /// Parses and returns a [`Stmt`].
    pub fn parse_stmt(&mut self) -> Stmt {
        let start_pos = self.start_pos();

        let kind = match self.peek() {
            TokenType::OpenBrace => self.parse_stmt_block(),
            TokenType::Print => self.parse_stmt_print(),
            _ => self.parse_stmt_expr(),
        };

        self.make_stmt(kind, start_pos)
    }

    /// Parses and returns a block [`StmtKind`].
    fn parse_stmt_block(&mut self) -> StmtKind {
        self.bump_assert(TokenType::OpenBrace);
        let mut decls = Vec::new();

        while !matches!(self.peek(), TokenType::Eof | TokenType::CloseBrace) {
            let decl = self.parse_decl();
            decls.push(decl);
        }

        self.expect(TokenType::CloseBrace);
        StmtKind::Block(decls.into_boxed_slice())
    }

    /// Parses and returns a print [`StmtKind`].
    fn parse_stmt_print(&mut self) -> StmtKind {
        self.bump_assert(TokenType::Print);
        let value = self.parse_expr();
        self.expect(TokenType::Semi);
        StmtKind::Print(Box::new(value))
    }

    /// Parses and returns an expression [`StmtKind`].
    fn parse_stmt_expr(&mut self) -> StmtKind {
        let expr = self.parse_expr();
        self.expect(TokenType::Semi);
        StmtKind::Expr(Box::new(expr))
    }

    /// Returns a new [`Stmt`] from a [`StmtKind`] and a start [`BytePos`].
    fn make_stmt(&self, kind: StmtKind, start_pos: BytePos) -> Stmt {
        let span = self.span_from(start_pos);
        Stmt { kind, span }
    }
}
