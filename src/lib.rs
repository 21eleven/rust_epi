mod fxn;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::fxn;
    use std::fs::File;
    use std::fs;
    use indicatif::{ProgressBar,ProgressStyle};
    //use std::time::Instant;
    use std::time::SystemTime;

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

    #[test]
    fn reverse_bits() {
        let name = "reverse_bits".into();
        let mut rdr = load_records(&name).expect("csv file");
        let rows = rdr.records();
        let length = count_tests(&name).expect("an int");
        let bar = ProgressBar::new(length as u64);
        let lookup = fxn::build_16bit_reverse_lookup();
        bar.set_style(ProgressStyle::default_bar()
            .template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"));
        for row in rows {
            let rec = row.expect("another record");
            let input = rec.get(0).unwrap().parse::<u64>().unwrap();
            let output = rec.get(1).unwrap().parse::<u64>().unwrap();
            let msg =rec.get(2).unwrap().parse::<String>().unwrap();

            let start = SystemTime::now();
            let result = fxn::reverse_bits(input, &lookup);
            let end = SystemTime::now();
            let runtime = end.duration_since(start).expect("a duraction");
            let s_run = runtime.as_nanos();
            let str_msg = stringify!(s_run);
            bar.set_message(&str_msg);

            if result == output {
                bar.inc(1);
            } else {
                bar.finish_with_message(&format!("err: {} -> {}, should be {}", input, result, output) as &str);
                assert_eq!(result, output);
            }
        }
        bar.finish_with_message("tests passed!");
    }

    #[test]
    fn parity_lookup() {
        let name = "parity".into();
        let mut rdr = load_records(&name).expect("csv file");
        let rows = rdr.records();
        let length = count_tests(&name).expect("an int");
        let bar = ProgressBar::new(length as u64);
        let lookup = fxn::build_16bit_parity_lookup();
        bar.set_style(ProgressStyle::default_bar()
            .template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"));
        for row in rows {
            let rec = row.expect("another record");
            let input = rec.get(0).unwrap().parse::<u64>().unwrap();
            let output = rec.get(1).unwrap().parse::<u8>().unwrap();
            let msg =rec.get(2).unwrap().parse::<String>().unwrap();

            let start = SystemTime::now();
            let result = fxn::parity_lookup(input, &lookup);
            let end = SystemTime::now();
            let runtime = end.duration_since(start).expect("a duraction");
            let s_run = runtime.as_nanos();
            let str_msg = &(format!("ns: {}", s_run ))[..];
            bar.set_message(&str_msg);

            if result == output {
                bar.inc(1);
            } else {
                bar.finish_with_message(&format!("err: {}",msg) as &str);
            }
        }
        bar.finish_with_message("tests passed!");

    }

    #[test]
    fn swap_bits() {
        let name = "swap_bits".into();
        let mut rdr = load_records(&name).expect("csv file");
        let rows = rdr.records();
        let length = count_tests(&name).expect("an int");
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
