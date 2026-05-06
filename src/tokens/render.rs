use std::fmt::{self, Formatter};

use crate::render::{Render, RenderContext};

use super::{Literal, Token, TokenKind, TokenType};

impl Render for Token {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.span.display(ctx), self.kind.display(ctx))
    }
}

impl Render for TokenKind {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => write!(f, "literal {}", literal.display(ctx)),
            Self::Ident(name) => write!(f, "identifier '{}'", name.display(ctx)),
            _ => write!(f, "{}", self.token_type().display(ctx)),
        }
    }
}

impl Render for TokenType {
    fn fmt(&self, _ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Render for Literal {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "{:?}", value.display(ctx).to_string()),
        }
    }
}
