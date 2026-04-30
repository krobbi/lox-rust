use std::fmt::{self, Formatter};

use crate::render::{Render, RenderContext};

/// A diagnostic message.
#[derive(Debug)]
pub enum Diag {
    /// A [`char`] which does not begin a [`Token`][crate::tokens::Token] was
    /// encountered.
    UnexpectedChar(char),
}

impl Render for Diag {
    fn fmt(&self, _ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(char) => write!(f, "Unexpected character {char:?}."),
        }
    }
}
