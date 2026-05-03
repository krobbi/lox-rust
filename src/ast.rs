use crate::{spans::Span, symbols::Symbol, tokens::Literal};

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

    /// A variable.
    Variable(Symbol),

    /// An instance.
    This,

    /// A superclass method.
    Super(Ident),

    /// A parenthesized [`Expr`].
    Paren(Box<Expr>),

    /// A unary operation.
    Unary(UnOp, Box<Expr>),

    /// A function or class call.
    Call(Box<Expr>, Box<[Expr]>),
}

/// A unary operator.
#[derive(Clone, Copy, Debug)]
pub enum UnOp {
    /// An arithmetic negation.
    Minus,

    /// A logical negation.
    Not,
}

/// An identifier [`Symbol`] with a [`Span`].
#[derive(Clone, Copy, Debug)]
pub struct Ident {
    /// The [`Symbol`].
    pub symbol: Symbol,

    /// The [`Span`].
    pub span: Span,
}
