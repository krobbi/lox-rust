use std::fmt::{self, Debug, Display, Formatter};

use crate::symbols::{Symbol, SymbolTable};

/// A trait for values which can be formatted with a [`RenderContext`].
pub trait Render: Debug {
    /// Creates a new [`RenderDisplay`] from the value and a [`RenderContext`].
    fn display<'sym>(&self, ctx: RenderContext<'sym>) -> RenderDisplay<'_, 'sym, Self> {
        RenderDisplay { value: self, ctx }
    }

    /// Formats the value with a [`RenderContext`] and a [`Formatter`]. This
    /// function returns a [`fmt::Error`] if a formatting error occurred.
    fn fmt(&self, ctx: RenderContext<'_>, f: &mut Formatter<'_>) -> fmt::Result;
}

/// A context for formatting a value.
#[derive(Clone, Copy)]
pub struct RenderContext<'sym> {
    /// The [`SymbolTable`].
    symbols: &'sym SymbolTable,
}

impl<'sym> RenderContext<'sym> {
    /// Creates a new `RenderContext` from a [`SymbolTable`].
    pub const fn new(symbols: &'sym SymbolTable) -> Self {
        Self { symbols }
    }

    /// Returns a [`Symbol`]'s string slice.
    pub fn symbol_string(self, symbol: Symbol) -> &'sym str {
        self.symbols.string(symbol)
    }
}

/// A structure which implements [`Display`] for values which implement
/// [`Render`].
#[derive(Clone, Copy)]
pub struct RenderDisplay<'val, 'sym, T: Render + ?Sized> {
    value: &'val T,

    /// The [`RenderContext`].
    ctx: RenderContext<'sym>,
}

impl<T: Render> Display for RenderDisplay<'_, '_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Render::fmt(self.value, self.ctx, f)
    }
}
