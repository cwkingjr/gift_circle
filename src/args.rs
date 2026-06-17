use std::path::PathBuf;

use clap::Parser;

/// Program to generate random gift assignments, with or without groups.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input CSV file of participants
    #[arg(long, short, value_name = "FILE")]
    pub input: PathBuf,
    #[arg(long, short, action)]
    pub use_groups: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
