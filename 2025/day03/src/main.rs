use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Bank = Vec<u8>;

fn get_max(slice: &[u8]) -> (usize, u8) {
    // Find the index and value of the FIRST maximum u8 in the slice
    slice
        .iter()
        .enumerate()
        .fold(None, |acc, (i, &v)| match acc {
            None => Some((i, v)),
            Some((_, max_v)) if v > max_v => Some((i, v)),
            _ => acc,
        })
        .expect("Cannot find max of empty slice")
}

fn get_joltage(bank: &Bank, num_batteries: usize) -> u64 {
    if bank.len() <= 1 || num_batteries == 0 {
        return 0;
    }

    // Choose num_batteries batteries, greedy from left to right, always picking the max in the current window.
    let mut result = Vec::with_capacity(num_batteries);
    let mut start_idx = 0;
    let len = bank.len();
    for b in 0..num_batteries {
        let batteries_left = num_batteries - b;
        let limit = len - batteries_left;
        let window = &bank[start_idx..=limit];
        let (rel_max_idx, max_val) = get_max(window);
        let abs_max_idx = start_idx + rel_max_idx;
        result.push(max_val);
        start_idx = abs_max_idx + 1;
    }
    // Compose the u64 by concatenating digits, i.e., [9, 8, 7] -> 987
    result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

fn part1(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| get_joltage(bank, 2)).sum()
}

fn part2(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| get_joltage(bank, 12)).sum()
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
