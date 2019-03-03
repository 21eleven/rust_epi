extern crate csv;

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[derive(Debug,Deserialize)]
struct Record {
    input: usize,
    output: u8,
    err_msg: String,
}


mod fxn;


#[cfg(test)]
mod tests {
    use std::error::Error;
    use csv::Reader;
    use super::fxn;

    use std::env;
    use std::ffi::OsString;
    use std::fs::File;
    use std::process;

    // By default, struct field names are deserialized based on the position of
    // a corresponding field in the CSV data's header record.
    
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
        //let file = "/home/noah/gits/rust_epi/tests/parity.tsv";
        //let mut rdr = csv::Reader::from_reader(file);
        //let mut rdr = csv::Reader::from_path(path);
        let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(file);
        for result in rdr.records() {
            let record = result?;
            let input = record.get(0).unwrap().parse::<i64>().unwrap();
            let output = record.get(1).unwrap().parse::<u8>().unwrap();
            let msg = record.get(2).unwrap();
            dbg!(input);
            dbg!(output);
            dbg!(msg);
            //println!("{:?}", record);
        }
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
