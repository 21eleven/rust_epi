mod fxn;
#[macro_use] extern crate  prettytable;

#[cfg(test)]
mod tests {
    use std::error::Error;
    use super::fxn;
    use std::fs::File;
    use std::fs;
    use indicatif::{ProgressBar,ProgressStyle};
    use std::time::Instant;
    //use prettytable::{Table, Row, Cell};
    
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
        fn print_results_table(&self) {
            let results = table!(
                ["Tests Passed", "Average Runtime", "Worst Runtime", "Best Runtime"],
                [bFGc->"✓", bFcc->format!("{:.3} {}", self.avg, self.unit), bFrc->format!("{} {}", self.worst, self.unit), bFmc->format!("{} {}", self.best, self.unit)]
            );
            results.printstd();
        }
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
    
    fn one_input_one_output_test_u64_w_lookup(name: &String, f: fn(u64, &Vec<u16>) -> u64, lookup_fxn: fn() -> Vec<u16>) {
        let mut rdr = load_records(&name).expect("csv file");
        let rows = rdr.records();
        let length = count_tests(&name).expect("an int");
        let lookup = lookup_fxn();
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
            let result = f(input, &lookup);
            let runtime = start.elapsed().as_nanos();
            stats.update(runtime);
            bar.set_message(stats.get_msg());

            if result == output {
                bar.inc(1);
            } else {
                println!("err: {} -> {}, should be {}", input, result, output);
                assert_eq!(result,output)
            }
        }
        bar.finish_with_message("tests passed!");
        stats.print_results_table();
    }
    fn three_input_one_output_test_u64(name: &String, f: fn(u64, u64, u64) -> u64) {
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
            let input_one = rec.get(0).unwrap().parse::<u64>().unwrap();
            let input_two = rec.get(1).unwrap().parse::<u64>().unwrap();
            let input_three = rec.get(2).unwrap().parse::<u64>().unwrap();
            let output = rec.get(3).unwrap().parse::<u64>().unwrap();
            let _msg =rec.get(4).unwrap().parse::<String>().unwrap();

            let start = Instant::now();
            let result = f(input_one, input_two, input_three);
            let runtime = start.elapsed().as_nanos();
            stats.update(runtime);
            bar.set_message(stats.get_msg());

            if result == output.into() {
                bar.inc(1);
            } else {
                println!("err: {}, {}, {} -> {}, should be {}", input_one, input_two, input_three, result, output);
                assert_eq!(result,output)
            }
        }
        bar.finish_with_message("tests passed!");
        println!("     {}", stats.msg);
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

            if result == output.into() {
                bar.inc(1);
            } else {
                println!("err: {} -> {}, should be {}", input, result, output);
                assert_eq!(result,output)
            }
        }
        bar.finish_with_message("tests passed!");
        stats.print_results_table();
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
        one_input_one_output_test_u64_w_lookup(&name, fxn::reverse_bits, fxn::build_16bit_reverse_lookup);
    }

    #[test]
    fn unoptimized_64bit_reversal() {
        let name = "reverse_bits".into();
        one_input_one_output_test_u64(&name, fxn::reverse_64);
    }

    #[test]
    fn parity_lookup() {
        let name = "parity".into();
        one_input_one_output_test_u64_w_lookup(&name, fxn::parity_lookup, fxn::build_16bit_parity_lookup);
    }

    #[test]
    fn swap_bits() {
        let name = "swap_bits".into();
        three_input_one_output_test_u64(&name, fxn::swap_bits);
    }

    #[test]
    fn parity() {
        let name = "parity".into();
        one_input_one_output_test_u64(&name, fxn::parity);
    }

}
