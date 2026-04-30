use std::fmt::{self, Formatter};

use crate::render::{Render, RenderContext};

use super::{BytePos, Span};

impl Render for Span {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        let (start_line, start_col) = byte_pos_to_source_pos(self.start, ctx.source());
        let (end_line, end_col) = byte_pos_to_source_pos(self.end, ctx.source());

        if end_line > start_line {
            write!(f, "[{start_line}:{start_col} - {end_line}:{end_col}]")
        } else if end_col > start_col {
            write!(f, "[{start_line}:{start_col}-{end_col}]")
        } else {
            write!(f, "[{start_line}:{start_col}]")
        }
    }
}

/// Converts a [`BytePos`] to a source position with source code.
fn byte_pos_to_source_pos(pos: BytePos, source: &str) -> (u32, u32) {
    let pos = pos.0.try_into().expect("target should be at least 32-bit");
    let mut line = 1;
    let mut col = 1;

    for (index, char) in source.char_indices() {
        if index >= pos {
            break;
        }

        (line, col) = add_char_to_source_pos(line, col, char);
    }

    (line, col)
}

/// Adds a [`char`] to a source position.
const fn add_char_to_source_pos(line: u32, col: u32, char: char) -> (u32, u32) {
    match char {
        '\t' => (line, col.next_multiple_of(4) + 1),
        '\n' => (line + 1, 1),
        '\r' => (line, col),
        _ => (line, col + 1),
    }
}
