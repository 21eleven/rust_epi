mod fxn;



#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::fxn;
    use std::fs::File;
    use std::fs;
    use indicatif::{ProgressBar,ProgressStyle};
    use std::time::Instant;
    use std::time::SystemTime;
    
    #[derive(Debug)]
    struct RuntimeStats {
        unit: String,
        runs: Vec<u128>,
        avg: f64,
        last: u128,
        best: u128,
        worst: u128,
        msg: String,
    }

    impl RuntimeStats {
        fn new(unit: String) -> RuntimeStats {
            RuntimeStats { unit: unit, runs: Vec::new(), avg: 0f64, last: 0u128, best: 1_000_000_000_000u128, worst: 0u128, msg: "ns: 0".into()}
        }
        fn add(&mut self, runtime: u128) {
            self.runs.push(runtime)
        }
        fn average(&mut self) {
            let sum: u128  = self.runs.iter().sum();
            self.avg = sum as f64 / self.runs.len() as f64
        }
        fn update(&mut self, runtime: u128) {
            self.add(runtime);
            self.last = runtime;
            if runtime > self.worst { self.worst = runtime }
            if runtime < self.best { self.best = runtime }
            self.average();
            self.msg = format!("| {} | {:>7} | avg: {:>7.3} | worst: {:>7} | best: {:>7} |", self.unit, self.last, self.avg, self.worst, self.best);
        }
        fn get_msg(&self) -> &str {
            &(self.msg)[..]
        }
    }

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
    fn one_input_one_output_test_u64(name: &String, f: fn(u64) -> u64) {
        let mut rdr = load_records(&name).expect("csv file");
        let rows = rdr.records();
        let length = count_tests(&name).expect("an int");
        let bar = ProgressBar::new(length as u64);
        let mut stats = RuntimeStats::new("ns".into());
        bar.set_style(ProgressStyle::default_bar()
            .template("[{spinner}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"));
        for row in rows {
            let rec = row.expect("another record");
            let input = rec.get(0).unwrap().parse::<u64>().unwrap();
            let output = rec.get(1).unwrap().parse::<u64>().unwrap();
            let _msg =rec.get(2).unwrap().parse::<String>().unwrap();

            let start = Instant::now();
            let result = f(input);
            let runtime = start.elapsed().as_nanos();
            stats.update(runtime);
            bar.set_message(stats.get_msg());

            if result == output {
                bar.inc(1);
            } else {
                //bar.set_message(&format!("err: {} -> {}, should be {}", input, result, output) as &str);
                println!("err: {} -> {}, should be {}", input, result, output);
                assert_eq!(result,output)
            }
        }
        bar.finish_with_message("tests passed!");
        println!("     {}", stats.msg);
    }

    #[test]
    fn closest_int_same_weight_o1_time_space_complexity() {
        let name = "closest_int_same_weight".into();
        one_input_one_output_test_u64(&name, fxn::closest_int_same_weight);
    }

    #[test]
    fn closest_int_same_weight() {
        let name = "closest_int_same_weight".into();
        one_input_one_output_test_u64(&name, fxn::closest_int_same_weight);
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
            let _msg =rec.get(2).unwrap().parse::<String>().unwrap();

            let start = SystemTime::now();
            let result = fxn::reverse_bits(input, &lookup);
            let end = SystemTime::now();
            let runtime = end.duration_since(start).expect("a duraction");
            let s_run = runtime.as_nanos();
            let str_msg = &(format!("ns: {}", s_run ))[..];
            bar.set_message(&str_msg);

            if result == output {
                bar.inc(1);
            } else {
                bar.set_message(&format!("err: {} -> {}, should be {}", input, result, output) as &str);
                assert_eq!(result,output)
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
                assert_eq!(result, output);
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
