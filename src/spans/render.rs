use std::fmt::{self, Display, Formatter};

use crate::log::{Render, RenderContext};

use super::{BytePos, Span};

impl Render for BytePos {
    fn fmt(&self, _ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Convert byte positions to line and column numbers.
        Display::fmt(&self.0, f)
    }
}

impl Render for Span {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} - {}]",
            self.start.display(ctx),
            self.end.display(ctx)
        )
    }
}
