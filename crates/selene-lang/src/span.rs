use std::fmt;

use miette::SourceSpan;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub const fn len(self) -> u32 {
        self.end - self.start
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        Self::new((span.start as usize).into(), (span.len() as usize).into())
    }
}
