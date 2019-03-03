extern crate csv;

extern crate serde;
#[macro_use]
extern crate serde_derive;



mod fxn;


#[cfg(test)]
mod tests {
    use std::error::Error;
    use csv::Reader;
    use super::fxn;
    use serde::Deserialize;
    use std::io::prelude::*;
    use std::path::Path;

    use std::env;
    use std::ffi::OsString;
    use std::fs::File;
    use std::process;
    // By default, struct field names are deserialized based on the position of
    // a corresponding field in the CSV data's header record.
    #[derive(Debug,Deserialize)]
    pub struct Record {
        pub input: usize,
        pub output: u8,
        pub err_msg: String,
    }
    
    //fn load_tests(path: String) -> Vec<(usize, usize, String)> {
    fn load_tests(path: String)  -> Result<(),Box<Error> >{
        let mut rows = Reader::from_path(path)?;
        dbg!("in fxn load_test");
        for row in rows.records() {
            let record = row?;
            println!("{:?}", record);
        }
        Ok(())
    }

    

    #[test]
    fn example() -> Result<(), Box<Error>> {
        dbg!("example");
        let file = File::open("/home/noah/gits/rust_epi/tests/parity.tsv")?;

        let mut data = String::new();
        file.read_to_string(&mut data)?;

        dbg!(data);
        //let file = "/home/noah/gits/rust_epi/tests/parity.tsv";
        //let mut rdr = csv::Reader::from_reader(file);
        //let mut rdr = csv::Reader::from_path(path);
        let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(data.as_bytes());
        //let mut iter = rdr.deserialize();
        for result in rdr.deserialize() {
            let record: Record = result?;
        }
        //if let Some(result) = iter.next() {
        //    let record: Record = result.unwrap();
        //}

        //for result in rdr.deserialize() {
        //for result in rdr.records() {
        // for result: Record in rdr.deserialize() {
            //dbg!(result);
        //    let row: super::Record = result?;
        //    dbg!(row);
        //}
        //let record: Row = result?;
        //for result in rdr.deserialize() {
        //    let record: Record = result?;
        //    dbg!(record);
            /*
            let input = record.
            let output = record.get(1).unwrap();
            dbg!(input);
            dbg!(output);
            //println!("{:?}", record);
            */
        //}
        Ok(())
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn parity() {
        dbg!("in parity test");
        let output = fxn::parity();
        let path = "./tests/parity.tsv".into();
        let _ = load_tests(path);
        assert_eq!(output, 1);
    }

}
