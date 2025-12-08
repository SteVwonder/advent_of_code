use std::error::Error;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
enum Cell {
    Entrance,
    Splitter,
    Empty,
}

impl Cell {
    fn from_char(c: char) -> Result<Cell, Box<dyn Error>> {
        match c { 'S' => Ok(Cell::Entrance), '^' => Ok(Cell::Splitter), _ => Ok(Cell::Empty) }
    }
}

type Grid = Vec<Vec<Cell>>;

fn parse_contents(contents: &str) -> Result<Grid, Box<dyn Error>> {
    let grid = contents.lines().map(|line| line.chars().map(|c| Cell::from_char(c).unwrap()).collect()).collect();
    Ok(grid)
}

fn part1(contents: &str) -> u64 {
    let grid = parse_contents(contents).unwrap();

    let starting_row = grid.iter().next().unwrap();
    let starting_idx = starting_row.iter().position(|c| c == &Cell::Entrance).unwrap();

    let mut current_row = HashSet::new();
    current_row.insert(starting_idx);

    let mut num_splits = 0;
    for row in grid.iter().skip(1) {
        let mut new_row = HashSet::new();
        for col_idx in current_row.iter() {
            if row[*col_idx] == Cell::Splitter {
                new_row.insert(col_idx + 1);
                new_row.insert(col_idx - 1);
                num_splits += 1;
            } else {
                new_row.insert(*col_idx);
            }
        }
        current_row = new_row;
    }
    num_splits as u64
}

fn part2(contents: &str) -> u64 {
0
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    println!("\tPart 1: {}", part1(&contents));
    println!("\tPart 2: {}", part2(&contents));
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
