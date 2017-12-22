extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::path::Path;
use log::LogLevel;

fn process(path: &Path) -> u32 {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b'\t')
        .from_path(path)
        .unwrap();
    let max_diffs: Vec<u32> = rdr.records()
        .map(|result| {
            let record = result.unwrap();
            debug!("{:?}", record);
            let integers: Vec<u32> = record
                .iter()
                .map(|val| val.parse::<u32>().unwrap())
                .collect();
            let max_diff = integers.iter().max().unwrap() - integers.iter().min().unwrap();
            debug!("Max diff: {}", max_diff);
            max_diff
        })
        .collect();
    max_diffs.iter().sum()
}

fn main() {
    env_logger::init().unwrap();

    let path = match log_enabled!(LogLevel::Debug) {
        true => Path::new("data/test_input"),
        false => Path::new("data/input"),
    };

    let sum = process(path);
    println!("The answer to part 1 is: {}", sum);
}
