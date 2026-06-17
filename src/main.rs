use std::io;
use std::process;

use anyhow::{Context, Result};

use gift_circle::{args::Args, get_gift_circle, People, Person};

fn run() -> Result<()> {
    let args = Args::parse_args();

    let mut rdr = csv::Reader::from_path(&args.input).with_context(|| {
        format!(
            "Failed to read input from {}",
            args.input.display()
        )
    })?;

    let people: People = rdr
        .deserialize::<Person>()
        .collect::<Result<Vec<Person>, _>>()?
        .into();

    let output = get_gift_circle(people, args.use_groups)?;

    if output.used_groups {
        eprintln!(
            "#INFO: Found valid gift circle USING groups in {} attempts",
            output.attempts
        );
    } else {
        eprintln!(
            "#INFO: Found valid gift circle NOT USING groups in {} attempts",
            output.attempts
        );
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());

    for person in output.people {
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
