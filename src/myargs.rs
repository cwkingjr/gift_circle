use clap::Parser;

/// Program to generate randowm gift circle from folks outside recipient's group
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input file
    #[arg(short, long)]
    pub input: String,
}

pub fn get_args() -> Args {
    let args = Args::parse();
    args
}
