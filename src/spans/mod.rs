mod render;

use std::ops::Add;

/// A byte index of source code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BytePos(u32);

impl BytePos {
    /// Creates a new zero `BytePos`.
    pub const fn new() -> Self {
        Self(0)
    }
}

impl Add<usize> for BytePos {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        #[expect(clippy::cast_possible_truncation, reason = "performance")]
        let rhs = rhs as u32;

        Self(self.0 + rhs)
    }
}

/// A span between a start and end [`BytePos`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    /// The start [`BytePos`].
    start: BytePos,

    /// The end [`BytePos`].
    end: BytePos,
}

impl Span {
    /// Creates a new `Span` from its start and end [`BytePos`].
    pub fn new(start: BytePos, end: BytePos) -> Self {
        debug_assert!(end >= start, "span is reversed");
        Self { start, end }
    }
}
