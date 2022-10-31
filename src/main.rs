mod gift_circle;
mod group;
mod myargs;
mod person;

use std::error::Error;
use std::io;
use std::process;

use gift_circle::get_gift_circle;
use myargs::get_args;
use person::Person;

fn run() -> Result<(), Box<dyn Error>> {
    let args = get_args();

    let mut rdr = csv::Reader::from_path(args.input)?;
    let mut wtr = csv::Writer::from_writer(io::stdout());

    let mut participants: Vec<Person> = vec![];

    for result in rdr.deserialize() {
        let person: Person = result?;
        participants.push(person);
    }

    let gift_circle = get_gift_circle(participants);
    for person in gift_circle {
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
