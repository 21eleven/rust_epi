extern crate csv;

mod fxn;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::fxn;
    use std::fs::File;

    fn load_tests(fxn_name: String)  -> Result< Vec<(usize, usize, String)>, Box<Error> > {
        let test_tsv = File::open(format!("./tests/{}.tsv", fxn_name))?;
        let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').has_headers(false).from_reader(test_tsv);
        let mut rows = Vec::new();
        for row in reader.records() {
            let record = row?;
            let input = record.get(0).unwrap().parse::<usize>().unwrap();
            let output = record.get(1).unwrap().parse::<usize>().unwrap();
            let msg = record.get(2).unwrap().parse::<String>().unwrap();
            rows.push((input, output, msg));
        }
        Ok(rows)
    }

    #[test]
    fn parity() {
        let name = "parity".into();
        let rows = load_tests(name).unwrap();
        let mut iter_row = rows.into_iter();
        loop {
            let r = iter_row.next().unwrap();
            let output = fxn::parity(r.0);
            assert_eq!(output, r.1);
            break;
        }
    }

}
