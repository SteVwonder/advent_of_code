extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate time;

use std::path::Path;
use log::LogLevel;
use time::PreciseTime;

fn part1(path: &Path) -> u32 {
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

fn part1(path: &Path) -> u32 {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .delimiter(b'\t')
        .from_path(path)
        .unwrap();
    let divisible_vals: Vec<u32> = rdr.records()
        .map(|result| {
            let record = result.unwrap();
            debug!("{:?}", record);
            let integers: Vec<u32> = record
                .iter()
                .map(|val| val.parse::<u32>().unwrap())
                .collect();
            find_divisble_vals
            debug!("Max diff: {}", max_diff);
            max_diff
        })
        .collect();
    divisible_vals.iter().sum()
}

fn timed_run(f: fn(&Path) -> u32, arg: &Path) -> u32{
    let start = PreciseTime::now();
    let sum = f(arg);
    let end = PreciseTime::now();
    let microsec = start.to(end).num_microseconds().unwrap_or(0);
    let millisec: f64 = (microsec as f64) / 1000.0;
    println!("Exec took {} ms", millisec);

    sum
}

fn main() {
    env_logger::init().unwrap();

    let path = match log_enabled!(LogLevel::Debug) {
        true => Path::new("data/test_input"),
        false => Path::new("data/input"),
    };

    let sum = timed_run(part1, path);
    println!("The answer to part 1 is: {}", sum);
    let sum = timed_run(part2, path);
    println!("The answer to part 2 is: {}", sum);
}
