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
        let expr = self.parse_expr();
        self.expect(TokenType::Semi);
        let kind = StmtKind::Expr(Box::new(expr));
        self.make_stmt(kind, start_pos)
    }

    /// Returns a new [`Stmt`] from a [`StmtKind`] and a start [`BytePos`].
    fn make_stmt(&self, kind: StmtKind, start_pos: BytePos) -> Stmt {
        let span = self.span_from(start_pos);
        Stmt { kind, span }
    }
}
