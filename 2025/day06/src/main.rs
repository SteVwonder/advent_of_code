use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

type Problem = (Vec<u64>, Op);
type Input = Vec<Problem>;

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Multiply),
            _ => Err(format!("Unknown operator: {}", s)),
        }
    }
}

fn part1(input: &Input) -> u64 {
    input.iter().map(|(nums, op)| {
        match op {
            Op::Add => nums.iter().sum::<u64>(),
            Op::Multiply => nums.iter().product::<u64>(),
        }
    }).sum()
}

fn part2(input: &Input) -> u64 {
0
}

fn parse_contents(contents: &str) -> Result<Input, Box<dyn Error>> {
    let lines = contents.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let num_problems = lines[0].len();
    let num_lines = lines.len();
    let mut problems: Input = Vec::with_capacity(num_problems);

    for i in 0..num_problems {
        let nums = lines[..num_lines - 1]
            .iter()
            .map(|row| row[i].parse::<u64>().unwrap())
            .collect();
        let op = lines[num_lines - 1][i].parse::<Op>().unwrap();
        problems.push((nums, op));
    }
    Ok(problems)
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    let input = parse_contents(&content)?;

    println!("\tPart 1: {}", part1(&input));
    println!("\tPart 2: {}", part2(&input));
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
