extern crate csv;

use std::error::Error;
use std::io;
use std::process;

fn example() -> Result<(), Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b':')
        .from_path("/home/noah/gits/rust_epi/tests/parity.tsv");
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
