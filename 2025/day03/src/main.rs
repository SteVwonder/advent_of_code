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
        .reduce(|acc, (i, v)| if v > acc.1 { (i, v) } else { acc })
        .map(|(i, &v)| (i, v))
        .expect("Cannot find max of empty slice")
}

fn get_joltage(bank: &Bank, num_batteries: usize) -> u64 {
    if bank.len() <= 1 || num_batteries == 0 {
        return 0;
    }

    // Choose num_batteries batteries, greedy from left to right
    // State: (start_idx, accumulated_result)
    let len = bank.len();
    (0..num_batteries)
        .fold((0, 0u64), |(start_idx, acc), b| {
            let batteries_left = num_batteries - b;
            let limit = len - batteries_left;
            let window = &bank[start_idx..=limit];
            let (rel_max_idx, max_val) = get_max(window);
            (start_idx + rel_max_idx + 1, acc * 10 + max_val as u64)
        })
        .1 // Extract the accumulated result
}

fn part1(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| get_joltage(bank, 2)).sum()
}

fn part2(banks: &[Bank]) -> u64 {
    banks.iter().map(|bank| get_joltage(bank, 12)).sum()
}

fn parse_line(line: &str) -> Bank {
    line.chars()
        .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
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
    // use super::*;
}
