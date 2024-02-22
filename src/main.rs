mod gift_circle;
mod group;
mod myargs;
mod people;
mod person;

use std::io;
use std::process;

use anyhow::{Context, Result};

use gift_circle::get_gift_circle;
use myargs::get_args;
use people::People;
use person::Person;

/// Processes args, reads the input csv file, generates gift circle,
/// converts gift circle to output csv, and writes output csv to standard out.
fn run() -> Result<()> {
    let args = get_args();

    let mut rdr = csv::Reader::from_path(args.input.clone())
        .with_context(|| format!("Failed to read input from {}", &args.input))?;

    let mut people: People = vec![];

    for result in rdr.deserialize() {
        let person: Person = result?;
        people.push(person);
    }

    let gift_circle: People = get_gift_circle(people, args.use_groups)?;

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for person in gift_circle {
        wtr.serialize(person)?;
    }

    Ok(wtr.flush()?)
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}
