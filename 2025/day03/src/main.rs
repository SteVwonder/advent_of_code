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

fn get_joltage(bank: &Bank) -> u64 {
    if bank.len() <= 1 {
        return 0;
    }

    let (max_idx, max_val) = get_max(bank);
    let (first_digit, second_digit) = if max_idx == bank.len() - 1 {
        (get_max(&bank[..max_idx]).1, max_val)
    } else {
        (max_val, get_max(&bank[max_idx + 1..]).1)
    };
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
