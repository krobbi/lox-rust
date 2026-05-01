#![expect(dead_code, reason = "fields are debug printed")]

use crate::{spans::Span, tokens::Literal};

/// An abstract syntax tree.
#[derive(Debug)]
pub struct Ast(pub Expr);

/// An expression.
#[derive(Debug)]
pub struct Expr {
    /// The [`ExprKind`].
    pub kind: ExprKind,

    /// The [`Span`].
    pub span: Span,
}

/// An [`Expr`]'s kind.
#[derive(Debug)]
pub enum ExprKind {
    /// A [`Literal`].
    Literal(Literal),
}
