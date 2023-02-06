mod gift_circle;
mod group;
mod myargs;
mod person;

use std::io;
use std::process;

use anyhow::{Context, Result};

use gift_circle::get_gift_circle;
use myargs::get_args;
use person::Person;

fn run() -> Result<()> {
    let args = get_args();

    let mut rdr = csv::Reader::from_path(args.input.clone())
        .with_context(|| format!("Failed to read input from {}", &args.input))?;

    let mut participants: Vec<Person> = vec![];

    for result in rdr.deserialize() {
        let person: Person = result?;
        participants.push(person);
    }

    let gift_circle = get_gift_circle(participants)?;

    if args.arrow_print {
        let mut names = gift_circle
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<_>>();
        // Add the first person to the end to wrap the circle
        let first_person = &names.first().unwrap().clone();
        names.push(first_person.to_string());

        println!("#{}", &names.join(" -> "));
    }

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
