use std::fmt::{self, Display, Formatter};

use super::{Literal, Token, TokenKind, TokenType};

impl Literal {
    /// Returns the `Literal`'s type name.
    const fn type_name(self) -> &'static str {
        match self {
            Self::Nil => "value", // Looks better than "nil 'nil'".
            Self::Bool(_) => "bool",
            Self::Number(_) => "number",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => {
                let type_name = literal.type_name();
                write!(f, "{type_name} '{literal}'")
            }
            Self::Ident(symbol) => write!(f, "identifier '{symbol}'"),
            _ => Display::fmt(&self.token_type(), f),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nil => f.write_str("nil"),
            Self::Bool(value) => Display::fmt(value, f),
            Self::Number(value) => Display::fmt(value, f),
        }
    }
}
