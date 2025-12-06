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

fn parse_contents_p1(contents: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let lines = contents.lines().map(|line| line.split_whitespace().collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let num_problems = lines[0].len();
    let num_lines = lines.len();
    let mut problems = Vec::with_capacity(num_problems);

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

fn part1(input: &str) -> u64 {
    let input = parse_contents_p1(input).unwrap();
    input.iter().map(|(nums, op)| {
        match op {
            Op::Add => nums.iter().sum::<u64>(),
            Op::Multiply => nums.iter().product::<u64>(),
        }
    }).sum()
}

/*
Cephalopod math is written right-to-left in columns. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)

Here's the example worksheet again:

123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  

Reading the problems right-to-left one column at a time, the problems are now quite different:

    The rightmost problem is 4 + 431 + 623 = 1058
    The second problem from the right is 175 * 581 * 32 = 3253600
    The third problem from the right is 8 + 248 + 369 = 625
    Finally, the leftmost problem is 356 * 24 * 1 = 8544
*/
fn parse_contents_p2(contents: &str) -> Result<Vec<Problem>, Box<dyn Error>> {
    let lines: Vec<&str> = contents.lines().collect();
    let num_lines = lines.len();

    // Find the maximum width
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Convert to 2D char grid, padding with spaces
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut chars: Vec<char> = l.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    let mut problems: Vec<Problem> = Vec::new();
    let mut current_cols: Vec<usize> = Vec::new();

    let process_problem = |cols: &[usize]| -> Problem {
        let mut nums: Vec<u64> = Vec::new();
        let mut op: Option<Op> = None;

        // Read columns right-to-left
        for &c in cols.iter().rev() {
            let bottom = grid[num_lines - 1][c];
            if bottom == '+' || bottom == '*' {
                op = Some(bottom.to_string().parse::<Op>().unwrap());
            }

            // Build number from column (top-to-bottom, excluding operator row)
            let num_str: String = (0..num_lines - 1)
                .map(|row| grid[row][c])
                .filter(|&ch| ch != ' ')
                .collect();

            if !num_str.is_empty() {
                nums.push(num_str.parse::<u64>().unwrap());
            }
        }

        (nums, op.unwrap())
    };

    for col in 0..width {
        let is_separator = (0..num_lines).all(|row| grid[row][col] == ' ');

        if is_separator {
            if !current_cols.is_empty() {
                problems.push(process_problem(&current_cols));
                current_cols.clear();
            }
        } else {
            current_cols.push(col);
        }
    }

    // Handle last problem if not followed by separator
    if !current_cols.is_empty() {
        problems.push(process_problem(&current_cols));
    }

    Ok(problems)
}

fn part2(input: &str) -> u64 {
    let input = parse_contents_p2(input).unwrap();
    input
        .iter()
        .map(|(nums, op)| match op {
            Op::Add => nums.iter().sum::<u64>(),
            Op::Multiply => nums.iter().product::<u64>(),
        })
        .sum()
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;

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
