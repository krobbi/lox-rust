use std::fmt::{self, Formatter};

use crate::{
    ast::{Ast, BinOp, Expr, ExprKind, Ident, LogicOp, Stmt, StmtKind, UnOp},
    render::{Render, RenderContext},
    spans::Span,
};

/// Prints an [`Ast`] with a [`RenderContext`].
pub fn print_ast(ast: &Ast, ctx: RenderContext<'_, '_>) {
    let node = Node::Ast(ast);
    let mut flags = Vec::new();
    print_node(node, ctx, &mut flags);
}

/// Recursively prints a [`Node`] and its children with a [`RenderContext`] and
/// indent flags.
fn print_node(node: Node<'_>, ctx: RenderContext<'_, '_>, flags: &mut Vec<bool>) {
    for (level, flag) in flags.iter().copied().enumerate() {
        let indent = match (level == flags.len() - 1, flag) {
            (false, false) => "│ ",
            (false, true) => "  ",
            (true, false) => "├─",
            (true, true) => "└─",
        };

        print!("{indent}");
    }

    println!("{}", node.display(ctx));
    let children = node.children();

    for (index, child) in children.iter().copied().enumerate() {
        flags.push(index == children.len() - 1);
        print_node(child, ctx, flags);
        flags.pop();
    }
}

/// A reference to an [`Ast`] node.
#[derive(Clone, Copy, Debug)]
enum Node<'ast> {
    /// An [`Ast`].
    Ast(&'ast Ast),

    /// A [`Stmt`].
    Stmt(&'ast Stmt),

    /// An [`Expr`].
    Expr(&'ast Expr),

    /// An [`Ident`].
    Ident(&'ast Ident),
}

impl Node<'_> {
    /// Returns the `Node`'s [`Span`]. This function returns [`None`] if the
    /// `Node` has no [`Span`].
    const fn span(self) -> Option<Span> {
        match self {
            Self::Ast(_) => None,
            Self::Stmt(Stmt { span, .. })
            | Self::Expr(Expr { span, .. })
            | Self::Ident(Ident { span, .. }) => Some(*span),
        }
    }

    /// Returns the `Node`'s child `Node`s.
    fn children(self) -> Vec<Self> {
        match self {
            Self::Ast(ast) => {
                let mut children = Vec::new();

                for decl in &ast.0 {
                    children.push(Node::Stmt(decl));
                }

                children
            }
            Self::Stmt(stmt) => stmt_children(stmt),
            Self::Expr(expr) => expr_children(expr),
            Self::Ident(_) => Vec::new(),
        }
    }
}

impl Render for Node<'_> {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        let result = match self {
            Self::Ast(_) => write!(f, "[Ast]"),
            Self::Stmt(stmt) => write!(f, "[Stmt] {}", stmt_name(stmt)),
            Self::Expr(expr) => fmt_expr(expr, ctx, f),
            Self::Ident(ident) => write!(f, "[Ident] {}", ident.name.display(ctx)),
        };

        if let Some(span) = self.span() {
            result?;
            write!(f, " {}", span.display(ctx))
        } else {
            result
        }
    }
}

/// Returns a [`Stmt`]'s child [`Node`]s.
fn stmt_children(stmt: &Stmt) -> Vec<Node<'_>> {
    match &stmt.kind {
        StmtKind::Block(decls) => {
            let mut children = Vec::new();

            for decl in decls {
                children.push(Node::Stmt(decl));
            }

            children
        }
        StmtKind::If(cond, then_stmt, Some(else_stmt)) => vec![
            Node::Expr(cond),
            Node::Stmt(then_stmt),
            Node::Stmt(else_stmt),
        ],
        StmtKind::If(cond, body, None) | StmtKind::While(cond, body) => {
            vec![Node::Expr(cond), Node::Stmt(body)]
        }
        StmtKind::Return(None) => Vec::new(),
        StmtKind::Return(Some(expr)) | StmtKind::Print(expr) | StmtKind::Expr(expr) => {
            vec![Node::Expr(expr)]
        }
    }
}

/// Returns an [`Expr`]'s child [`Node`]s.
fn expr_children(expr: &Expr) -> Vec<Node<'_>> {
    match &expr.kind {
        ExprKind::AssignVar(ident, expr) => vec![Node::Ident(ident), Node::Expr(expr)],
        ExprKind::AssignField(instance, name, value) => {
            vec![Node::Expr(instance), Node::Ident(name), Node::Expr(value)]
        }
        ExprKind::Literal(_) | ExprKind::Variable(_) | ExprKind::This => Vec::new(),
        ExprKind::Property(expr, ident) => vec![Node::Expr(expr), Node::Ident(ident)],
        ExprKind::Super(ident) => vec![Node::Ident(ident)],
        ExprKind::Paren(expr) | ExprKind::Unary(_, expr) => vec![Node::Expr(expr)],
        ExprKind::Binary(_, lhs, rhs) | ExprKind::Logic(_, lhs, rhs) => {
            vec![Node::Expr(lhs), Node::Expr(rhs)]
        }
        ExprKind::Call(callee, args) => {
            let mut children = vec![Node::Expr(callee)];

            for arg in args {
                children.push(Node::Expr(arg));
            }

            children
        }
    }
}

/// Returns a [`Stmt`]'s name.
const fn stmt_name(stmt: &Stmt) -> &'static str {
    match &stmt.kind {
        StmtKind::Block(_) => "Block",
        StmtKind::If(_, _, _) => "If",
        StmtKind::While(_, _) => "While",
        StmtKind::Return(_) => "Return",
        StmtKind::Print(_) => "Print",
        StmtKind::Expr(_) => "Expr",
    }
}

/// Returns a [`UnOp`]'s name.
const fn un_op_name(op: UnOp) -> &'static str {
    match op {
        UnOp::Minus => "Minus",
        UnOp::Not => "Not",
    }
}

/// Returns a [`BinOp`]'s name.
const fn bin_op_name(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "Add",
        BinOp::Subtract => "Subtract",
        BinOp::Multiply => "Multiply",
        BinOp::Divide => "Divide",
        BinOp::Equal => "Equal",
        BinOp::NotEqual => "NotEqual",
        BinOp::Greater => "Greater",
        BinOp::GreaterEqual => "GreaterEqual",
        BinOp::Less => "Less",
        BinOp::LessEqual => "LessEqual",
    }
}

/// Returns a [`LogicOp`]'s name.
const fn logic_op_name(op: LogicOp) -> &'static str {
    match op {
        LogicOp::And => "And",
        LogicOp::Or => "Or",
    }
}

/// Formats an [`Expr`] with a [`RenderContext`] and a [`Formatter`]. This
/// function returns a [`fmt::Error`] if a formatting error occurred.
fn fmt_expr(expr: &Expr, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[Expr] ")?;

    match &expr.kind {
        ExprKind::AssignVar(_, _) => write!(f, "AssignVar"),
        ExprKind::AssignField(_, _, _) => write!(f, "AssignField"),
        ExprKind::Literal(literal) => write!(f, "Literal({})", literal.display(ctx)),
        ExprKind::Variable(name) => write!(f, "Variable({})", name.display(ctx)),
        ExprKind::Property(_, _) => write!(f, "Property"),
        ExprKind::This => write!(f, "This"),
        ExprKind::Super(_) => write!(f, "Super"),
        ExprKind::Paren(_) => write!(f, "Paren"),
        ExprKind::Unary(op, _) => write!(f, "Unary({})", un_op_name(*op)),
        ExprKind::Binary(op, _, _) => write!(f, "Binary({})", bin_op_name(*op)),
        ExprKind::Logic(op, _, _) => write!(f, "Logic({})", logic_op_name(*op)),
        ExprKind::Call(_, _) => write!(f, "Call"),
    }
}
