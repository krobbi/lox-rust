use std::fmt::{self, Formatter};

use crate::{
    render::{Render, RenderContext},
    tokens::{TokenKind, TokenType},
};

/// A diagnostic message.
#[derive(Debug)]
pub enum Diag {
    /// A [`char`] which does not begin a [`Token`][crate::tokens::Token] was
    /// encountered.
    UnexpectedChar(char),

    /// A string literal without a terminating quote was encountered.
    UnterminatedString,

    /// A number literal with a leading decimal point was encountered.
    LeadingDecimal,

    /// A number literal with a trailing decimal point was encountered.
    TrailingDecimal,

    /// A [`TokenKind`] which does not match an expected [`TokenType`] was
    /// encountered.
    UnexpectedToken(TokenType, TokenKind),

    /// A [`TokenKind`] which does not begin an [`Expr`][crate::ast::Expr] was
    /// encountered.
    ExpectedExpr(TokenKind),

    /// An invalid assignment target was used.
    InvalidAssign,
}

impl Render for Diag {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "Unexpected character {char:?}."),
            Self::UnterminatedString => write!(f, "This string has no terminating '\"'."),
            Self::LeadingDecimal => write!(f, "This number starts with '.' - add a leading '0'."),
            Self::TrailingDecimal => write!(
                f,
                "This number has a trailing '.' - remove it or add a trailing '0'."
            ),
            Self::ExpectedExpr(kind) => {
                write!(f, "Expected an expression, found {}.", kind.display(ctx))
            }
            Self::UnexpectedToken(token_type, kind) => write!(
                f,
                "Expected {}, found {}.",
                token_type.display(ctx),
                kind.display(ctx)
            ),
            Self::InvalidAssign => write!(f, "This is not a valid assignment target."),
        }
    }
}
