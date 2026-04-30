use std::fmt::{self, Debug, Display, Formatter};

use crate::log::{Render, RenderContext};

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
            Self::Ident(symbol) => write!(f, "identifier '{}'", symbol.display(ctx)),
            _ => Render::fmt(&self.token_type(), ctx, f),
        }
    }
}

impl Render for TokenType {
    fn fmt(&self, _ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl Render for Literal {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => f.write_str("nil"),
            Self::Bool(value) => Display::fmt(value, f),
            Self::Number(value) => Display::fmt(value, f),
            Self::String(symbol) => Debug::fmt(&symbol.display(ctx).to_string(), f),
        }
    }
}
