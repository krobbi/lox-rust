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
}
