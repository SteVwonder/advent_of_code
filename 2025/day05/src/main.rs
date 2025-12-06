use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::ops::Range;

fn point_in_ranges(point: u64, ranges: &[Range<u64>]) -> bool {
    ranges.iter().any(|range| range.contains(&point))
}

fn part1(ranges: &[Range<u64>], points: &[u64]) -> u64 {
    points.iter().filter(|&point| point_in_ranges(*point, ranges)).count() as u64
}

fn part2(ranges: &[Range<u64>], points: &[u64]) -> u64 {
0
}

fn parse_range(line: &str) -> Range<u64> {
    let (start, end) = line.split_once('-').unwrap();
    Range {
        start: start.parse().unwrap(),
        // the problem states the ranges are inclusive on both ends, Range is exclusive on the end
        // so we add 1 to the end to make it inclusive
        end: end.parse::<u64>().unwrap() + 1,
    }
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut ranges = Vec::new();

    // Collect all lines until empty line using take_while
    ranges.extend(
        lines
            .by_ref()
            .map(|l| l.unwrap())
            .take_while(|line| !line.trim().is_empty())
            .map(|line| parse_range(&line))
    );

    // Collect remaining lines into point until EOF in a functional way
    let points: Vec<u64> = lines
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();


    println!("\tPart 1: {}", part1(&ranges, &points));
    println!("\tPart 2: {}", part2(&ranges, &points));
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
