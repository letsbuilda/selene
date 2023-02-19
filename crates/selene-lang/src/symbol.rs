use std::{fmt, sync::LazyLock};

use lasso::{Spur, ThreadedRodeo};

static INTERNER: LazyLock<ThreadedRodeo> = LazyLock::new(ThreadedRodeo::new);

/// An interned string.
///
/// Note: This should not be used for large strings. It's only intended for
/// usage with small strings that have a high chance of being repeated.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Symbol(Spur);

impl From<&str> for Symbol {
    fn from(s: &str) -> Self {
        Self(INTERNER.get_or_intern(s))
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Self {
        Self(INTERNER.get_or_intern(s))
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", INTERNER.resolve(&self.0))
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", INTERNER.resolve(&self.0))
    }
}
