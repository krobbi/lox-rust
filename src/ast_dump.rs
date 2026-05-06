use std::fmt::{self, Formatter};

use crate::{
    ast::{Ast, BinOp, Expr, ExprKind, Ident, LogicOp, Stmt, StmtKind, UnOp},
    render::{Render, RenderContext},
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

/// An [`Ast`] node.
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
        match self {
            Self::Ast(_) => write!(f, "[Ast]"),
            Self::Stmt(stmt) => fmt_stmt(stmt, ctx, f),
            Self::Expr(expr) => fmt_expr(expr, ctx, f),
            Self::Ident(ident) => write!(
                f,
                "[Ident]{} {}",
                ident.span.display(ctx),
                ident.name.display(ctx)
            ),
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

/// Formats a [`Stmt`] with a [`RenderContext`] and a [`Formatter`]. This
/// function returns a [`fmt::Error`] if a formatting error occurred.
fn fmt_stmt(stmt: &Stmt, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[Stmt]{} ", stmt.span.display(ctx))?;

    match &stmt.kind {
        StmtKind::Block(_) => write!(f, "Block"),
        StmtKind::If(_, _, _) => write!(f, "If"),
        StmtKind::While(_, _) => write!(f, "While"),
        StmtKind::Return(_) => write!(f, "Return"),
        StmtKind::Print(_) => write!(f, "Print"),
        StmtKind::Expr(_) => write!(f, "Expr"),
    }
}

/// Formats an [`Expr`] with a [`RenderContext`] and a [`Formatter`]. This
/// function returns a [`fmt::Error`] if a formatting error occurred.
fn fmt_expr(expr: &Expr, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[Expr]{} ", expr.span.display(ctx))?;

    match &expr.kind {
        ExprKind::AssignVar(_, _) => write!(f, "AssignVar"),
        ExprKind::AssignField(_, _, _) => write!(f, "AssignField"),
        ExprKind::Literal(literal) => write!(f, "Literal({})", literal.display(ctx)),
        ExprKind::Variable(name) => write!(f, "Variable({})", name.display(ctx)),
        ExprKind::Property(_, _) => write!(f, "Property"),
        ExprKind::This => write!(f, "This"),
        ExprKind::Super(_) => write!(f, "Super"),
        ExprKind::Paren(_) => write!(f, "Paren"),
        ExprKind::Unary(op, _) => write!(f, "Unary({})", unary_symbol(*op)),
        ExprKind::Binary(op, _, _) => write!(f, "Binary({})", binary_symbol(*op)),
        ExprKind::Logic(op, _, _) => write!(f, "Logic({})", logic_symbol(*op)),
        ExprKind::Call(_, _) => write!(f, "Call"),
    }
}

/// Returns a [`char`] for displaying a [`UnOp`].
const fn unary_symbol(op: UnOp) -> char {
    match op {
        UnOp::Minus => '-',
        UnOp::Not => '!',
    }
}

/// Returns a string slice for displaying a [`BinOp`].
const fn binary_symbol(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Subtract => "-",
        BinOp::Multiply => "*",
        BinOp::Divide => "/",
        BinOp::Equal => "==",
        BinOp::NotEqual => "!=",
        BinOp::Greater => ">",
        BinOp::GreaterEqual => ">=",
        BinOp::Less => "<",
        BinOp::LessEqual => "<=",
    }
}

/// Returns a string slice for displaying a [`LogicOp`].
const fn logic_symbol(op: LogicOp) -> &'static str {
    match op {
        LogicOp::And => "and",
        LogicOp::Or => "or",
    }
}
