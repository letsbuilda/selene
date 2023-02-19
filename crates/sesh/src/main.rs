//! The binary entrypoint for the sesh shell.

#![warn(clippy::pedantic, rust_2018_idioms)]
#![allow(dead_code)]

mod cli;
mod command;
mod config;
mod logger;

use std::fs;

use clap::Parser;
use cli::Cli;
use miette::{IntoDiagnostic, NamedSource, Report, Result};

fn main() -> Result<()> {
    let args = Cli::parse();
    let contents = fs::read_to_string(&args.file).into_diagnostic()?;

    selene_lang::execute(&contents).map_err(|errors| {
        Report::from(errors)
            .with_source_code(NamedSource::new(args.file.display().to_string(), contents))
    })?;

    Ok(())
}
