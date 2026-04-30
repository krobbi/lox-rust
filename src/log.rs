use std::fmt::{self, Formatter};

use crate::{
    diagnostics::Diag,
    render::{Render, RenderContext},
    spans::Span,
};

/// A log of [`Record`]s.
#[derive(Default)]
pub struct Log {
    /// The [`Record`]s.
    records: Vec<Record>,
}

impl Log {
    /// Creates a new `Log`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reports a [`Diag`] at a [`Span`].
    #[cold]
    pub fn report(&mut self, diag: Diag, span: Span) {
        self.records.push(Record { diag, span });
    }

    /// Displays and clears the `Log` with a [`RenderContext`].
    pub fn flush(&mut self, ctx: RenderContext<'_, '_>) {
        for record in &self.records {
            eprintln!("{}", record.display(ctx));
        }

        self.records.clear();
    }
}

/// A recorded [`Diag`] with a [`Span`].
#[derive(Debug)]
struct Record {
    /// The [`Diag`].
    diag: Diag,

    /// The [`Span`].
    span: Span,
}

impl Render for Record {
    fn fmt(&self, ctx: RenderContext<'_, '_>, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Error]{} {}",
            self.span.display(ctx),
            self.diag.display(ctx)
        )
    }
}
