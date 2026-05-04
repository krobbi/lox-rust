#[cfg(test)]
mod tests;

use std::str::Chars;

/// A structure which reads a stream of [`char`]s from source code.
pub struct Scanner<'src> {
    /// The source code's [`Chars`].
    chars: Chars<'src>,

    /// The string slice between the start of the current lexeme and the end of
    /// source code.
    rest: &'src str,
}

impl<'src> Scanner<'src> {
    /// Creates a new `Scanner` from source code.
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
            rest: source,
        }
    }

    /// Returns the current lexeme's length in bytes.
    pub fn lexeme_length(&self) -> usize {
        self.rest.len() - self.chars.as_str().len()
    }

    /// Returns the current lexeme.
    pub fn lexeme(&self) -> &'src str {
        let length = self.lexeme_length();

        #[expect(
            clippy::string_slice,
            reason = "length is always on a code point boundary"
        )]
        &self.rest[..length]
    }

    /// Begins a new lexeme.
    pub fn begin_lexeme(&mut self) {
        self.rest = self.chars.as_str();
    }

    /// Returns the next [`char`] without consuming it. This function returns
    /// [`None`] if the `Scanner` is at the end of source code.
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Returns the next pair of [`char`]s without consuming them. This function
    /// returns [`None`] for a [`char`] if the `Scanner` is at the end of source
    /// code.
    pub fn peek_pair(&self) -> (Option<char>, Option<char>) {
        let mut chars = self.chars.clone();
        (chars.next(), chars.next())
    }

    /// Consumes and returns the next [`char`]. This function returns [`None`]
    /// if the `Scanner` is at the end of source code.
    pub fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Consumes the next [`char`] if it matches an expected [`char`]. This
    /// function returns [`true`] if a [`char`] was consumed.
    pub fn eat(&mut self, expected: char) -> bool {
        let is_match = self.peek() == Some(expected);

        if is_match {
            self.bump();
        }

        is_match
    }

    /// Repeatedly consumes the next [`char`] while it matches a predicate
    /// function.
    pub fn eat_while<F: Fn(char) -> bool>(&mut self, predicate: F) {
        while let Some(char) = self.peek()
            && predicate(char)
        {
            self.bump();
        }
    }
}
