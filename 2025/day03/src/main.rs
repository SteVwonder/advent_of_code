use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Bank = Vec<u8>;

fn get_max(bank: &Bank, start_idx: usize, end_idx: usize) -> (usize, u8) {
    // Find the index of the maximum u8 value in the slice of the bank from start_idx (inclusive) to end_idx (exclusive)
    if start_idx >= bank.len() || end_idx > bank.len() || start_idx >= end_idx {
        panic!(
            "Invalid range: start_idx {} end_idx {} for bank of len {}",
            start_idx, end_idx, bank.len()
        );
    }
    let mut max_val = None;
    let mut max_idx = start_idx;
    for (i, &v) in bank.iter().enumerate().take(end_idx).skip(start_idx) {
        if max_val.is_none() || v > max_val.unwrap() {
            max_val = Some(v);
            max_idx = i;
        }
    }
    (max_idx, max_val.unwrap())
}

fn get_joltage(bank: &Bank) -> u64 {
    let mut first_digit = 0;
    let mut second_digit = 0;

    let (max_idx, max_val) = get_max(bank, 0, bank.len());
    if max_idx == bank.len() - 1 {
        first_digit = get_max(bank, 0, max_idx).1;
        second_digit = max_val;
    } else {
        first_digit = max_val;
        second_digit = get_max(bank, max_idx + 1, bank.len()).1;
    }
    first_digit as u64 * 10 + second_digit as u64
}

fn part1(banks: &[Bank]) -> u64 {
    banks.iter().map(get_joltage).sum()
}

fn part2(banks: &[Bank]) -> u64 {
    0
}

// line contains a list of digits (e.g., 1234542398593)
fn parse_line(line: &str) -> Bank {
    line.chars()
        .map(|battery| {
            let parsed = battery.to_digit(10)
                .expect("Invalid battery character") as u8;
            if parsed <= 9 {
                parsed
            } else {
                panic!("Battery digit out of range 0-9: {}", parsed)
            }
        })
        .collect()
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let banks = reader
        .lines()
        .map(|line| {
            let line = line?;
            Ok(parse_line(&line))
        })
        .collect::<Result<Vec<Bank>, std::io::Error>>()?;

    println!("\tPart 1: {}", part1(&banks));
    println!("\tPart 2: {}", part2(&banks));
    Ok(())
}

fn main() {
    println!("===Test===");
    if let Err(e) = solve(Path::new("./test")) {
        eprintln!("Error solving test: {}", e);
    }
    println!();
    println!("===Input===");
    if let Err(e) = solve(Path::new("./input")) {
        eprintln!("Error solving input: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
