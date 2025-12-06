use std::error::Error;
use std::fs;
use std::ops::Range;
use std::path::Path;

fn point_in_ranges(point: u64, ranges: &[Range<u64>]) -> bool {
    ranges.iter().any(|range| range.contains(&point))
}

fn part1(ranges: &[Range<u64>], points: &[u64]) -> u64 {
    points.iter().filter(|&point| point_in_ranges(*point, ranges)).count() as u64
}

fn part2(ranges: &mut [Range<u64>]) -> u64 {
    ranges.sort_by_key(|r| r.start);

    let mut covered_until = 0;
    let mut total_coverage = 0;
    for range in ranges.iter() {
        let effective_start = covered_until.max(range.start);
        total_coverage += range.end.saturating_sub(effective_start);
        covered_until = covered_until.max(range.end);
    }
    total_coverage
}

fn parse_range(line: &str) -> Result<Range<u64>, Box<dyn Error>> {
    let (start, end) = line.split_once('-')
        .ok_or("Invalid range format: missing '-'")?;
    Ok(Range {
        start: start.parse()?,
        // the problem states the ranges are inclusive on both ends, Range is exclusive on the end
        // so we add 1 to the end to make it inclusive
        end: end.parse::<u64>()? + 1,
    })
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let mut sections = content.split("\n\n");

    let mut ranges: Vec<Range<u64>> = sections
        .next()
        .ok_or("Missing ranges section")?
        .lines()
        .map(|line| parse_range(line))
        .collect::<Result<_, _>>()?;

    let points: Vec<u64> = sections
        .next()
        .ok_or("Missing points section")?
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    println!("\tPart 1: {}", part1(&ranges, &points));
    println!("\tPart 2: {}", part2(&mut ranges));
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
