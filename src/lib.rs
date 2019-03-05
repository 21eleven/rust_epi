extern crate csv;
extern crate indicatif;

mod fxn;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::fxn;
    use std::fs::File;
    use indicatif::{ProgressBar,ProgressStyle};

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
        let length = rows.len();
        //let mut counter = 1;
        let mut iter_row = rows.into_iter();
        let bar = ProgressBar::new(length as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"));
        loop {
            match iter_row.next() {
                Some(r) => {
                    let output = fxn::parity(r.0);
                    assert_eq!(output, r.1);
                    bar.inc(1);
                },
                None => {break},
            }
        }
    }

}
