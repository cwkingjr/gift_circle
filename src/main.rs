mod gift_path;

use std::error::Error;
use std::io;
use std::process;

use clap::Parser;

use gift_path::{get_gift_path, Person};

/// Program to generate randowm gift circle from folks outside recipient's group
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Input file
   #[arg(short, long)]
   input: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut rdr = csv::Reader::from_path(args.input)?;
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let mut participants: Vec<Person> = vec![];

    for result in rdr.deserialize() {
        let person: Person = result?;
        participants.push(person);
    }

    let mypath = get_gift_path(participants);
    for person in mypath {
        wtr.serialize(person)?;
    }

    match wtr.flush() {
        // convert the io error into a box dyn error to match return type
        Err(e) => Err(Box::from(e)),
        _ => Ok(()),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
