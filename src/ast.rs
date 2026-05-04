use crate::{spans::Span, symbols::Symbol, tokens::Literal};

/// An abstract syntax tree.
#[derive(Debug)]
pub struct Ast(pub Box<[Stmt]>);

/// A statement.
#[derive(Debug)]
pub struct Stmt {
    /// The [`StmtKind`].
    pub kind: StmtKind,

    /// The [`Span`].
    pub span: Span,
}

/// A [`Stmt`]'s kind.
#[derive(Debug)]
pub enum StmtKind {
    /// A block.
    Block(Box<[Stmt]>),

    /// A conditional branch.
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),

    /// A print.
    Print(Box<Expr>),

    /// An [`Expr`].
    Expr(Box<Expr>),
}

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
    /// A variable assignment.
    AssignVar(Ident, Box<Expr>),

    /// A field assignment.
    AssignField(Box<Expr>, Ident, Box<Expr>),

    /// A [`Literal`].
    Literal(Literal),

    /// A variable.
    Variable(Symbol),

    /// A property.
    Property(Box<Expr>, Ident),

    /// An instance.
    This,

    /// A superclass method.
    Super(Ident),

    /// A parenthesized [`Expr`].
    Paren(Box<Expr>),

    /// A unary operation.
    Unary(UnOp, Box<Expr>),

    /// A binary operation.
    Binary(BinOp, Box<Expr>, Box<Expr>),

    /// A logical operation.
    Logic(LogicOp, Box<Expr>, Box<Expr>),

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

/// A binary operator.
#[derive(Clone, Copy, Debug)]
pub enum BinOp {
    /// An addition or string concatenation.
    Add,

    /// A subtraction.
    Subtract,

    /// A multiplication.
    Multiply,

    /// A division.
    Divide,

    /// An equality comparison.
    Equal,

    /// An inequality comparison.
    NotEqual,

    /// A greater than comparison.
    Greater,

    /// A greater than or equal to comparison.
    GreaterEqual,

    /// A less than comparison.
    Less,

    /// A less than or equal to comparison.
    LessEqual,
}

/// A logical operator.
#[derive(Clone, Copy, Debug)]
pub enum LogicOp {
    /// A logical and.
    And,

    /// A logical or.
    Or,
}

/// An identifier [`Symbol`] with a [`Span`].
#[derive(Clone, Copy, Debug)]
pub struct Ident {
    /// The [`Symbol`].
    pub symbol: Symbol,

    /// The [`Span`].
    pub span: Span,
}
