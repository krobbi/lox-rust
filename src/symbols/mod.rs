#[cfg(test)]
mod tests;

mod render;

use std::collections::HashMap;

/// A unique identifier for an interned string slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(u32);

/// A table of [`Symbol`]s.
#[derive(Default)]
pub struct SymbolTable {
    /// The map of string slices to [`Symbol`]s.
    symbols: HashMap<Box<str>, Symbol>,

    /// The index for the next [`Symbol`].
    next_index: u32,
}

impl SymbolTable {
    /// Creates a new `SymbolTable`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a [`Symbol`]'s string slice.
    pub fn string(&self, symbol: Symbol) -> &str {
        #[expect(clippy::iter_over_hash_type, reason = "symbols are unique")]
        for (string, interned_symbol) in &self.symbols {
            if *interned_symbol == symbol {
                return string;
            }
        }

        unreachable!("symbol should have a string slice");
    }

    /// Interns a string slice and returns its [`Symbol`].
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(symbol) = self.symbols.get(string).copied() {
            return symbol;
        }

        let symbol = Symbol(self.next_index);
        let old_symbol = self.symbols.insert(Box::from(string), symbol);
        debug_assert!(old_symbol.is_none(), "string slice is already interned");
        debug_assert!(self.next_index < u32::MAX, "symbol table is full");
        self.next_index += 1;
        symbol
    }
}
