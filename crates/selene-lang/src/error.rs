#![expect(clippy::module_name_repetitions, reason = "the re-exports will be most commonly used")]

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Debug, Default, Error, Diagnostic)]
#[error("execution failed with {} error{}", errors.len(), if errors.len() == 1 { "" } else { "s" })]
pub struct ErrorSink {
    #[related]
    errors: Vec<LangError>,
}

impl ErrorSink {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn push(&mut self, error: LangError) {
        self.errors.push(error);
    }
}

#[derive(Debug, Diagnostic, Error)]
pub enum LangError {
    #[error("unterminated string literal")]
    UnterminatedString {
        #[label("string starts here and continues until the end of the file")]
        span: SourceSpan,
    },

    #[error("float literal without fractional digits")]
    #[diagnostic(help("consider adding a `0` after the decimal point"))]
    FloatWithoutFractional {
        #[label("float literal is here")]
        span: SourceSpan,
    },

    #[error("unknown numeric suffix `{suffix}`")]
    #[diagnostic(help(
        "the known suffixes are:\n\
        sizes: `b`, `kb`, `mb`, `gb`, `kib`, `mib`, `gib`\n\
        durations: `day`, `hr`, `min`, `sec`, `ms`, `us`, `ns`",
    ))]
    UnknownNumericSuffix {
        suffix: String,
        #[label("suffix used here")]
        span: SourceSpan,
    },
}
