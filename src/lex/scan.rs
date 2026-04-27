use std::str::Chars;

/// A structure which reads a stream of [`char`]s from source code.
pub struct Scanner<'src> {
    /// The source code's [`Chars`].
    chars: Chars<'src>,
}

impl<'src> Scanner<'src> {
    /// Creates a new `Scanner` from source code.
    pub fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars(),
        }
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

    /// Returns the next [`char`] without consuming it. This function returns
    /// [`None`] if the `Scanner` is at the end of source code.
    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
}
