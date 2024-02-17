use clap::Parser;

/// Program to generate random gift assignments, with or without groups.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(long, short)]
    pub input: String,
    #[clap(long, short, action)]
    pub use_groups: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
