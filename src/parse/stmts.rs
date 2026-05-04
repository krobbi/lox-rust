use crate::{
    ast::{Stmt, StmtKind},
    tokens::TokenType,
};

use super::Parser;

impl Parser<'_, '_, '_> {
    /// Parses and returns a [`Stmt`].
    pub fn parse_stmt(&mut self) -> Stmt {
        let start_pos = self.start_pos();

        let kind = match self.peek() {
            TokenType::OpenBrace => self.parse_stmt_block(),
            TokenType::If => self.parse_stmt_if(),
            TokenType::Print => self.parse_stmt_print(),
            TokenType::Return => self.parse_stmt_return(),
            TokenType::While => self.parse_stmt_while(),
            _ => self.parse_stmt_expr(),
        };

        let span = self.span_from(start_pos);
        Stmt { kind, span }
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

    /// Parses and returns an if [`StmtKind`].
    fn parse_stmt_if(&mut self) -> StmtKind {
        self.bump_assert(TokenType::If);
        self.expect(TokenType::OpenParen);
        let cond = self.parse_expr();
        self.expect(TokenType::CloseParen);
        let then_stmt = self.parse_stmt();
        let else_stmt = self
            .eat(TokenType::Else)
            .then(|| Box::new(self.parse_stmt()));

        StmtKind::If(Box::new(cond), Box::new(then_stmt), else_stmt)
    }

    /// Parses and returns a while [`StmtKind`].
    fn parse_stmt_while(&mut self) -> StmtKind {
        self.bump_assert(TokenType::While);
        self.expect(TokenType::OpenParen);
        let cond = self.parse_expr();
        self.expect(TokenType::CloseParen);
        let body = self.parse_stmt();
        StmtKind::While(Box::new(cond), Box::new(body))
    }

    /// Parses and returns a return [`StmtKind`].
    fn parse_stmt_return(&mut self) -> StmtKind {
        self.bump_assert(TokenType::Return);

        let value = (!self.eat(TokenType::Semi)).then(|| {
            let value = self.parse_expr();
            self.expect(TokenType::Semi);
            Box::new(value)
        });

        StmtKind::Return(value)
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
}
