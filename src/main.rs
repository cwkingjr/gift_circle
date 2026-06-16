use std::io;
use std::process;

use anyhow::{Context, Result};

use gift_circle::{get_gift_circle, myargs::get_args, People, Person};

fn run() -> Result<()> {
    let args = get_args();

    let mut rdr = csv::Reader::from_path(args.input.clone())
        .with_context(|| format!("Failed to read input from {}", &args.input))?;

    let people: People = rdr
        .deserialize::<Person>()
        .collect::<Result<Vec<Person>, _>>()?
        .into();

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
