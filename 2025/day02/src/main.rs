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
fn is_invalid_id_part1(id: i64) -> bool {
    let num_digits = id.ilog10() + 1;
    if num_digits % 2 != 0 {
        return false;
    }
    is_invalid_id_part2_helper(id, num_digits / 2)
}

fn sum_of_invalid_ids_in_range<F>(id_range: &IDRange, is_invalid_id: F) -> i64
where
    F: Fn(i64) -> bool,
{
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
        sum_of_invalid_ids += sum_of_invalid_ids_in_range(range, is_invalid_id_part1);
    }
    sum_of_invalid_ids
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
fn is_invalid_id_part2(id: i64) -> bool {
    let num_digits = id.ilog10() + 1;
    for size in 1..=num_digits/2 {
        if is_invalid_id_part2_helper(id, size) {
            return true;
        }
    }
    false
}

fn is_invalid_id_part2_helper(id: i64, size: u32) -> bool {
    let num_digits = id.ilog10() + 1;
    if num_digits % size != 0 {
        return false;
    }
    let pattern = id % 10_i64.pow(size);
    for i in 0..num_digits/size {
        let modulus = 10_i64.pow((i+1) * size);
        let divisor = 10_i64.pow(i * size);
        let digits_to_check = (id % modulus) / divisor;
        if digits_to_check != pattern {
            return false;
        }
    }
    true
}

fn part2(id_ranges: &[IDRange]) -> i64 {
    let mut sum_of_invalid_ids = 0;
    for range in id_ranges {
        sum_of_invalid_ids += sum_of_invalid_ids_in_range(range, is_invalid_id_part2);
    }
    sum_of_invalid_ids
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
        // Also include 1110 explicitly as a valid ID
        let mut valid_ids: Vec<i64> = (1698522..=1698528).collect();
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
        let valid_ids: Vec<i64> = (1698522..=1698528).collect();

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
