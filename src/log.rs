use std::fmt::{self, Debug, Display, Formatter};

use crate::symbols::{Symbol, SymbolTable};

/// A trait for values which can be formatted with a [`RenderContext`].
pub trait Render: Debug {
    /// Creates a new [`RenderDisplay`] from the value and a [`RenderContext`].
    fn display<'src, 'sym>(
        &self,
        ctx: RenderContext<'src, 'sym>,
    ) -> RenderDisplay<'_, 'src, 'sym, Self> {
        RenderDisplay { value: self, ctx }
    }

    /// Formats the value with a [`RenderContext`] and a [`Formatter`]. This
    /// function returns a [`fmt::Error`] if a formatting error occurred.
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result;
}

/// A context for formatting a value.
#[derive(Clone, Copy)]
pub struct RenderContext<'src, 'sym> {
    /// The source code.
    #[expect(dead_code, reason = "field should be used later")]
    source: &'src str,

    /// The [`SymbolTable`].
    symbols: &'sym SymbolTable,
}

impl<'src, 'sym> RenderContext<'src, 'sym> {
    /// Creates a new `RenderContext` from source code and a [`SymbolTable`].
    pub const fn new(source: &'src str, symbols: &'sym SymbolTable) -> Self {
        Self { source, symbols }
    }

    /// Returns a [`Symbol`]'s string slice.
    pub fn symbol_string(self, symbol: Symbol) -> &'sym str {
        self.symbols.string(symbol)
    }
}

/// A structure which implements [`Display`] for values which implement
/// [`Render`].
#[derive(Clone, Copy)]
pub struct RenderDisplay<'val, 'src, 'sym, T: Render + ?Sized> {
    value: &'val T,

    /// The [`RenderContext`].
    ctx: RenderContext<'src, 'sym>,
}

impl<T: Render> Display for RenderDisplay<'_, '_, '_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Render::fmt(self.value, self.ctx, f)
    }
}
