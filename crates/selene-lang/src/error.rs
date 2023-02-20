#![expect(clippy::module_name_repetitions, reason = "the re-exports will be most commonly used")]

use miette::Diagnostic;
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
pub enum LangError {}
