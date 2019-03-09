//extern crate csv;
//extern crate indicatif;

mod fxn;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::fxn;
    use std::fs::File;
    use std::fs;
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
    /*fn load_tests2(fxn_name: String)  -> Result< Vec<T>, Box<Error> > {
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
    }*/
    
    fn load_records(fxn_name: &String) -> Result<csv::Reader<std::fs::File>, Box<Error>> {
        dbg!("in load fxn");
        let tsv = File::open(format!("./tests/{}.tsv", &fxn_name))?;
        let reader = csv::ReaderBuilder::new().delimiter(b'\t').flexible(true).has_headers(true).from_reader(tsv);
        Ok(reader)
    }

    fn count_tests(fxn_name: &String) -> Result<usize, Box<Error>> {
        let tsv_string = fs::read_to_string(format!("./tests/{}.tsv", &fxn_name))?;
        let byte_vec = tsv_string.into_bytes();
        let n_lines = byte_vec.into_iter().filter(|x| x == &b'\n').count();
        Ok(n_lines)
    }

    #[parity_lookup]
    fn parity_lookup() {
        
    }

    #[test]
    fn swap_bits() {
        let name = "swap_bits".into();
        let mut rdr = load_records(&name).expect("csv file");
        dbg!("loaded");
        let rows = rdr.records();
        let length = count_tests(&name).expect("an int");
        dbg!(length);
        let bar = ProgressBar::new(length as u64);
        bar.set_style(ProgressStyle::default_bar()
            .template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"));
        for row in rows {
            let rec = row.expect("another record");
            let input = rec.get(0).unwrap().parse::<usize>().unwrap();
            let idx1 = rec.get(1).unwrap().parse::<u8>().unwrap();
            let idx2 = rec.get(2).unwrap().parse::<u8>().unwrap();
            let output = rec.get(3).unwrap().parse::<usize>().unwrap();
            let msg =rec.get(4).unwrap().parse::<String>().unwrap();

            let result = fxn::swap_bits(input, idx1, idx2);
            if result == output {
                bar.inc(1);
            } else {
                bar.finish_with_message(&format!("err: {}",msg) as &str);
            }
        }
        bar.finish_with_message("tests passed!");
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
                    assert_eq!(output as usize, r.1);
                    bar.inc(1);
                },
                None => {break},
            }
        }
    }

}
