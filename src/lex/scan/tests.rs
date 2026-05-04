use super::*;

/// Tests that [`Scanner::peek_pair`] returns pairs in order.
#[test]
fn peek_pair_is_ordered() {
    let scanner = Scanner::new("ab");
    assert_eq!(scanner.peek_pair(), (Some('a'), Some('b')));
}

/// Tests that [`Scanner::peek_pair`] supports partial pairs at the end of
/// source code.
#[test]
fn peek_pair_supports_partial_pairs() {
    let scanner = Scanner::new("a");
    assert_eq!(scanner.peek_pair(), (Some('a'), None));
}

/// Tests that [`Scanner::peek_pair`] supports empty pairs at the end of source
/// code.
#[test]
fn peek_pair_supports_empty_pairs() {
    let scanner = Scanner::new("");
    assert_eq!(scanner.peek_pair(), (None, None));
}
