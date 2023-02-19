use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    pub file: PathBuf,
}
