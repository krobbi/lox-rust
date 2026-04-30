use std::fmt::{self, Formatter};

use crate::log::{Render, RenderContext};

use super::Symbol;

impl Render for Symbol {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(ctx.symbol_string(*self))
    }
}
