use std::hint;

use super::*;

/// Tests that the pre-interned [`Symbol`]s have the expected string slices.
#[test]
fn pre_interned_symbols_have_expected_string_slices() {
    let symbols = SymbolTable::new();
    assert_eq!(symbols.string(Symbol::ERROR), "<error>");
}

/// Tests that string slices are interned to the expected pre-interned symbols.
#[test]
fn string_slices_intern_to_expected_pre_interned_symbols() {
    let mut symbols = SymbolTable::new();
    assert_eq!(symbols.intern("<error>"), Symbol::ERROR);
}

/// Tests that the first interned [`Symbol`] has index zero.
#[test]
fn first_symbol_has_index_zero() {
    let mut symbols = SymbolTable::new_empty();
    assert_eq!(symbols.intern("first"), Symbol(0));
}

/// Tests that [`Symbol`]s are interned with incrementing indices.
#[test]
fn symbols_have_incrementing_indices() {
    let mut symbols = SymbolTable::new();
    let base = symbols.intern("test::0").0;
    assert_eq!(symbols.intern("test::1"), Symbol(base + 1));
    assert_eq!(symbols.intern("test::2"), Symbol(base + 2));
    assert_eq!(symbols.intern("test::0"), Symbol(base));
    assert_eq!(symbols.intern("no_correlation;;1337"), Symbol(base + 3));
}

/// Tests that [`Symbol`]s preserve the string slices they were interned from.
#[test]
fn symbols_preserve_string_slices() {
    let mut symbols = SymbolTable::new();
    let first = symbols.intern("foo");
    let second = symbols.intern("bar");
    let third = symbols.intern("baz");
    assert_eq!(symbols.string(first), "foo");
    assert_eq!(symbols.string(second), "bar");
    assert_eq!(symbols.string(third), "baz");
}

/// Tests that [`Symbol`]s interned from equal string slices are equal.
#[test]
fn symbols_preserve_equality() {
    let mut symbols = SymbolTable::new();
    let mut string = String::new();
    string.push(hint::black_box('f'));
    string.push(hint::black_box('o'));
    string.push(hint::black_box('o'));
    assert_eq!(symbols.intern(&string), symbols.intern("foo"));
}

/// Tests that [`Symbol`]s interned from unequal string slices are unequal.
#[test]
fn symbols_preserve_inequality() {
    let mut symbols = SymbolTable::new();
    let mut string = String::from("foo");
    let original = symbols.intern(&string);
    string.pop();
    string.push('r');
    assert_ne!(original, symbols.intern(&string));
}

/// Tests that [`Symbol`]s are case-sensitive.
#[test]
fn symbols_are_case_sensitive() {
    let mut symbols = SymbolTable::new();
    let lower = symbols.intern("foo");
    let accent = symbols.intern("fóó");
    let title = symbols.intern("Foo");
    let upper = symbols.intern("FOO");
    assert_ne!(lower, accent);
    assert_ne!(lower, title);
    assert_ne!(lower, upper);
    assert_ne!(accent, title);
    assert_ne!(accent, upper);
    assert_ne!(title, upper);
}

/// Tests that [`Symbol`]s are length-sensitive.
#[test]
fn symbols_are_length_sensitive() {
    let mut symbols = SymbolTable::new();
    let empty = symbols.intern("");
    let one = symbols.intern("f");
    let short = symbols.intern("foo");
    let long = symbols.intern("food");
    let longer = symbols.intern("foods");
    assert_ne!(empty, one);
    assert_ne!(empty, short);
    assert_ne!(empty, long);
    assert_ne!(empty, longer);
    assert_ne!(one, short);
    assert_ne!(one, long);
    assert_ne!(one, longer);
    assert_ne!(short, long);
    assert_ne!(short, longer);
    assert_ne!(long, longer);
}
