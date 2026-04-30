use std::fmt::{self, Formatter};

use crate::render::{Render, RenderContext};

use super::Symbol;

impl Render for Symbol {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", ctx.symbol_string(*self))
    }
}
