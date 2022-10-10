mod gift_path;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::io;
use std::process;

use gift_path::{get_gift_path, Person};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::Reader::from_path(file_path)?;
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

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from(
            "expected 1 argument referencing the input file path, but got none",
        )),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
