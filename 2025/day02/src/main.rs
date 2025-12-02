use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct IDRange {
    start: u64,
    end: u64,
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
fn is_invalid_id_part1(id: u64) -> bool {
    if id == 0 {
        return false;
    }
    let num_digits = id.ilog10() + 1;
    if num_digits % 2 != 0 {
        return false;
    }
    has_repeated_pattern_of_size(id, num_digits / 2)
}

fn sum_of_invalid_ids_in_range<F>(id_range: &IDRange, is_invalid_id: &F) -> u64
where
    F: Fn(u64) -> bool,
{
    (id_range.start..=id_range.end)
        .filter(|&id| is_invalid_id(id))
        .sum()
}

fn solve_part<F>(id_ranges: &[IDRange], is_invalid: F) -> u64
where
    F: Fn(u64) -> bool,
{
    id_ranges
        .iter()
        .map(|range| sum_of_invalid_ids_in_range(range, &is_invalid))
        .sum()
}

fn part1(id_ranges: &[IDRange]) -> u64 {
    solve_part(id_ranges, is_invalid_id_part1)
}

/*
Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice.
So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:

    11-22 still has two invalid IDs, 11 and 22.
    95-115 now has two invalid IDs, 99 and 111.
    998-1012 now has two invalid IDs, 999 and 1010.
    1188511880-1188511890 still has one invalid ID, 1188511885.
    222220-222224 still has one invalid ID, 222222.
    1698522-1698528 still contains no invalid IDs.
    446443-446449 still has one invalid ID, 446446.
    38593856-38593862 still has one invalid ID, 38593859.
    565653-565659 now has one invalid ID, 565656.
    824824821-824824827 now has one invalid ID, 824824824.
    2121212118-2121212124 now has one invalid ID, 2121212121.
*/
fn is_invalid_id_part2(id: u64) -> bool {
    if id == 0 {
        return false;
    }
    let num_digits = id.ilog10() + 1;
    (1..=num_digits / 2).any(|size| has_repeated_pattern_of_size(id, size))
}

fn has_repeated_pattern_of_size(id: u64, size: u32) -> bool {
    let num_digits = id.ilog10() + 1;
    if num_digits % size != 0 {
        return false;
    }
    let pattern = id % 10_u64.pow(size);
    (0..num_digits / size).all(|i| {
        let modulus = 10_u64.pow((i + 1) * size);
        let divisor = 10_u64.pow(i * size);
        let digits_to_check = (id % modulus) / divisor;
        digits_to_check == pattern
    })
}

fn part2(id_ranges: &[IDRange]) -> u64 {
    solve_part(id_ranges, is_invalid_id_part2)
}

// line contains all ranges, comma separated, e.g. 11-22,95-115,998-1012
fn parse_line(line: &str) -> Vec<IDRange> {
    line.split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').expect("Invalid range format");
            IDRange {
                start: start.parse().unwrap(),
                end: end.parse().unwrap(),
            }
        })
        .collect()
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let line = reader.lines().next().ok_or("Empty file")??;
    let id_ranges = parse_line(&line);

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
        // Also include 1110 explicitly as a valid ID
        let mut valid_ids: Vec<u64> = (1698522..=1698528).collect();
        valid_ids.push(1110);
        valid_ids.push(555);

        for id in &invalid_ids {
            assert!(
                is_invalid_id_part1(*id),
                "Expected {} to be invalid, but got false",
                id
            );
        }

        for id in &valid_ids {
            assert!(
                !is_invalid_id_part1(*id),
                "Expected {} to be valid, but got true",
                id
            );
        }
    }

    #[test]
    fn test_is_invalid_id_part2() {
        // Invalid IDs from the comment examples (should return true)
        let invalid_ids = vec![
            12341234,   // 1234 two times
            123123123,  // 123 three times
            1212121212, // 12 five times
            1111111,    // 1 seven times
            11,         // from range 11-22
            22,         // from range 11-22
            99,         // from range 95-115
            111,        // from range 95-115
            999,        // from range 998-1012
            1010,       // from range 998-1012
            1188511885, // from range 1188511880-1188511890
            222222,     // from range 222220-222224
            446446,     // from range 446443-446449
            38593859,   // from range 38593856-38593862
            565656,     // from range 565653-565659
            824824824,  // from range 824824821-824824827
            2121212121, // from range 2121212118-2121212124
        ];

        // Valid IDs (range 1698522-1698528 contains no invalid IDs, should return false)
        let valid_ids: Vec<u64> = (1698522..=1698528).collect();

        for id in &invalid_ids {
            assert!(
                is_invalid_id_part2(*id),
                "Expected {} to be invalid, but got false",
                id
            );
        }

        for id in &valid_ids {
            assert!(
                !is_invalid_id_part2(*id),
                "Expected {} to be valid, but got true",
                id
            );
        }
    }
}
