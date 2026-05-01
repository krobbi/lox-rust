use std::fmt::{self, Formatter};

use crate::{
    ast::{Ast, Expr, ExprKind},
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
            (false, false) => '│',
            (false, true) => ' ',
            (true, false) => '├',
            (true, true) => '└',
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

    /// An [`Expr`].
    Expr(&'ast Expr),
}

impl Node<'_> {
    /// Returns the `Node`'s child `Node`s.
    fn children(self) -> Vec<Self> {
        match self {
            Self::Ast(ast) => vec![Node::Expr(&ast.0)],
            Self::Expr(expr) => expr_children(expr),
        }
    }
}

/// Rerturns an [`Expr`]'s child `Node`s.
const fn expr_children(expr: &Expr) -> Vec<Node<'_>> {
    match expr.kind {
        ExprKind::Literal(_) | ExprKind::Variable(_) => Vec::new(),
    }
}

impl Render for Node<'_> {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ast(_) => write!(f, "[Ast]"),
            Self::Expr(expr) => fmt_expr(expr, ctx, f),
        }
    }
}

/// Formats an [`Expr`] with a [`RenderContext`] and a [`Formatter`]. This
/// function returns a [`fmt::Error`] if a formatting error occurred.
fn fmt_expr(expr: &Expr, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "[Expr]{} ", expr.span.display(ctx))?;

    match expr.kind {
        ExprKind::Literal(literal) => write!(f, "Literal({})", literal.display(ctx)),
        ExprKind::Variable(symbol) => write!(f, "Variable({})", symbol.display(ctx)),
    }
}
