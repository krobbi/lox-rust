use std::mem;

use crate::{
    ast::{Ast, Expr, ExprKind},
    diagnostics::Diag,
    lex::Lexer,
    log::Log,
    symbols::SymbolTable,
    tokens::{Literal, Token, TokenKind},
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
}

impl<'src, 'sym, 'log> Parser<'src, 'sym, 'log> {
    /// Creates a new `Paser` from source code, a [`SymbolTable`], and a
    /// [`Log`].
    fn new(source: &'src str, symbols: &'sym mut SymbolTable, log: &'log mut Log) -> Self {
        let mut lexer = Lexer::new(source, symbols, log);
        let next_token = lexer.next_token();
        Self { lexer, next_token }
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
        let span = token.span();

        let kind = match token.kind() {
            TokenKind::Literal(literal) => ExprKind::Literal(literal),
            kind => {
                self.lexer.log_mut().report(Diag::ExpectedExpr(kind), span);
                error_expr()
            }
        };

        Expr { kind, span }
    }

    /// Consumes and returns the next [`Token`].
    fn bump(&mut self) -> Token {
        mem::replace(&mut self.next_token, self.lexer.next_token())
    }
}

/// Returns a new synthetic [`ExprKind`] for error recovery.
const fn error_expr() -> ExprKind {
    ExprKind::Literal(Literal::Nil)
}
