use std::hint;

use super::*;

/// Tests that string slices are interned with sequential [`Symbol`] indices.
#[test]
fn string_slices_are_interned_sequentially() {
    // Use an empty symbol table to avoid indices from pre-interned symbols.
    let mut symbols = SymbolTable {
        symbols: HashMap::new(),
        next_index: 0,
    };

    assert_eq!(symbols.intern("a"), Symbol(0));
    assert_eq!(symbols.intern("b"), Symbol(1));
    assert_eq!(symbols.intern("c"), Symbol(2));
}

/// Tests that [`Symbol`]s are equal if they are interned from the same string
/// slice.
#[test]
fn symbols_preserve_equality() {
    let mut symbols = SymbolTable::new();

    let foo_a = symbols.intern("foo");
    let foo_upper = symbols.intern("Foo");
    let foo_b = symbols.intern("foo");
    let bar = symbols.intern("bar");

    assert_eq!(foo_a, foo_b);
    assert_ne!(foo_a, foo_upper);
    assert_ne!(foo_a, bar);
    assert_ne!(foo_upper, bar);

    // Test that string slices are checked for equality by value.
    let mut string = String::new();
    string.push(hint::black_box('f'));
    string.push(hint::black_box('o'));
    string.push(hint::black_box('o'));

    assert_eq!(symbols.intern(&string), foo_a);
}
