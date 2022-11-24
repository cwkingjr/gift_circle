use clap::Parser;

/// Program to generate randowm gift circle from folks outside recipient's group
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(long, short)]
    pub input: String,
    #[clap(long, short, action)]
    pub arrow_print: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
