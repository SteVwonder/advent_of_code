use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct IDRange {
    start: i64,
    end: i64,
}

/*
An invalid ID is any ID which is made only of some sequence of digits repeated twice.
So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.
11-22 has two invalid IDs, 11 and 22.
95-115 has one invalid ID, 99.
998-1012 has one invalid ID, 1010.
1188511880-1188511890 has one invalid ID, 1188511885.
222220-222224 has one invalid ID, 222222.
1698522-1698528 contains no invalid IDs.
446443-446449 has one invalid ID, 446446.
38593856-38593862 has one invalid ID, 38593859.
*/
fn is_invalid_id(id: i64) -> bool {
    let num_digits = id.ilog10() + 1;
    if num_digits % 2 != 0 {
        return false;
    }
    let low_order_digits = id % 10_i64.pow(num_digits / 2);
    let high_order_digits = id / 10_i64.pow(num_digits / 2);
    low_order_digits == high_order_digits
}

fn sum_of_invalid_ids_in_range(id_range: &IDRange) -> i64 {
    let mut sum_of_invalid_ids = 0;
    for id in id_range.start..=id_range.end {
        if is_invalid_id(id) {
            sum_of_invalid_ids += id;
        }
    }
    sum_of_invalid_ids
}

fn part1(id_ranges: &[IDRange]) -> i64 {
    let mut sum_of_invalid_ids = 0;
    for range in id_ranges {
        sum_of_invalid_ids += sum_of_invalid_ids_in_range(range);
    }
    sum_of_invalid_ids
}

fn part2(id_ranges: &[IDRange]) -> i64 {
    0
}

// line contains all ranges, comma separated, e.g. 11-22,95-115,998-1012
fn parse_line(line: &str) -> Vec<IDRange> {
    let ranges = line.split(',').map(|range| {
        let parts = range.split('-').map(|part| part.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        IDRange {
            start: parts[0],
            end: parts[1],
        }
    }).collect::<Vec<IDRange>>();
    ranges
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let line = reader
        .lines()
        .take(1)
        .collect::<Result<Vec<String>, _>>()?;
    let id_ranges = parse_line(&line[0]);

    println!("\tPart 1: {}", part1(&id_ranges));
    println!("\tPart 2: {}", part2(&id_ranges));
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

    #[test]
    fn test_is_invalid_id() {
        // Invalid IDs from the comment examples (should return true)
        let invalid_ids = vec![
            55,         // 5 twice
            6464,       // 64 twice
            123123,     // 123 twice
            11,         // from range 11-22
            22,         // from range 11-22
            99,         // from range 95-115
            1010,       // from range 998-1012
            1188511885, // from range 1188511880-1188511890
            222222,     // from range 222220-222224
            446446,     // from range 446443-446449
            38593859,   // from range 38593856-38593862
        ];

        // Valid IDs (range 1698522-1698528 contains no invalid IDs, should return false)
        let valid_ids: Vec<i64> = (1698522..=1698528).collect();

        for id in &invalid_ids {
            assert!(
                is_invalid_id(*id),
                "Expected {} to be invalid, but got false",
                id
            );
        }

        for id in &valid_ids {
            assert!(
                !is_invalid_id(*id),
                "Expected {} to be valid, but got true",
                id
            );
        }
    }
}
