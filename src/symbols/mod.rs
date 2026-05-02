#[cfg(test)]
mod tests;

mod render;

use std::collections::HashMap;

/// An interned identifier for a string slice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Symbol(u32);

impl Symbol {
    /// A synthetic `Symbol` for error recovery.
    pub const ERROR: Self = Self(0);
}

/// A table of [`Symbol`]s.
pub struct SymbolTable {
    /// The map of string slices to [`Symbol`]s.
    symbols: HashMap<Box<str>, Symbol>,

    /// The index for the next [`Symbol`].
    next_index: u32,
}

impl SymbolTable {
    /// Creates a new `SymbolTable`.
    pub fn new() -> Self {
        let mut symbols = Self::new_empty();
        symbols.pre_intern("<error>", Symbol::ERROR);
        symbols
    }

    /// Returns a [`Symbol`]'s string slice.
    pub fn string(&self, symbol: Symbol) -> &str {
        #[expect(clippy::iter_over_hash_type, reason = "symbols are interned")]
        for (string, interned_symbol) in &self.symbols {
            if *interned_symbol == symbol {
                return string;
            }
        }

        unreachable!("symbol should have a string slice");
    }

    /// Interns and returns a [`Symbol`] from a string slice.
    pub fn intern(&mut self, string: &str) -> Symbol {
        if let Some(symbol) = self.symbols.get(string).copied() {
            return symbol;
        }

        let symbol = Symbol(self.next_index);
        let old_symbol = self.symbols.insert(Box::from(string), symbol);
        debug_assert!(old_symbol.is_none(), "symbol is already interned");
        debug_assert!(self.next_index < u32::MAX, "symbol table is full");
        self.next_index += 1;
        symbol
    }

    /// Creates a new `SymbolTable` without any pre-interned [`Symbol`]s.
    fn new_empty() -> Self {
        Self {
            symbols: HashMap::new(),
            next_index: 0,
        }
    }

    /// Pre-interns a [`Symbol`] from a string slice and an expected [`Symbol`].
    fn pre_intern(&mut self, string: &'static str, expected_symbol: Symbol) {
        let symbol = self.intern(string);

        debug_assert_eq!(
            symbol, expected_symbol,
            "pre-interned symbol does not match expected symbol"
        );

        debug_assert_eq!(
            self.string(symbol),
            string,
            "pre-interned symbol does not match string slice"
        );
    }
}
