//! The Selene language.

#![feature(lint_reasons, once_cell)]
#![warn(clippy::pedantic, rust_2018_idioms)]
#![expect(
    clippy::must_use_candidate,
    reason = "as an application, there's not a lot of value in this"
)]

pub mod error;
mod lexer;
mod span;
mod symbol;

use error::ErrorSink;

/// Execute the given source code as a whole.
///
/// TODO: This is a placeholder for now. Eventually execution will have a more
/// interactive API.
///
/// # Errors
///
/// If any errors are encountered, they will be returned as a `Result::Err`.
/// Otherwise, execution can be assumed to have succeeded.
pub fn execute(source: &str) -> Result<(), ErrorSink> {
    let (_, errors) = lexer::lex(source);

    if errors.has_errors() {
        Err(errors)
    } else {
        Ok(())
    }
}
