use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use ndarray::Array2;
use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};

fn parse_grid(filename: &Path) -> Result<Array2<u8>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let rows: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| if c == '@' { 1 } else { 0 })
                .collect()
        })
        .collect();
    
    let nrows = rows.len();
    let ncols = rows.first().map(|r| r.len()).unwrap_or(0);
    
    let flat: Vec<u8> = rows.into_iter().flatten().collect();
    let grid = Array2::from_shape_vec((nrows, ncols), flat)?;
    
    Ok(grid)
}

fn get_accessible_rolls(grid: &Array2<u8>) -> Array2<u8> {
    let kernel = array![
        [1, 1, 1],
        [1, 10, 1],
        [1, 1, 1],
    ];
    let convolution = grid.conv(&kernel, ConvMode::Same, PaddingMode::Zeros).unwrap();
    convolution.map(|&x| if x >= 10 && x < 14 { 1 } else { 0 })
}

fn count_accessible_rolls(accessible_rolls: &Array2<u8>) -> u64 {
    accessible_rolls.iter().fold(0u64, |acc, &x| acc + x as u64)
}

fn part1(grid: &Array2<u8>) -> u64 {
    count_accessible_rolls(&get_accessible_rolls(grid))
}

fn part2(grid: &Array2<u8>) -> u64 {
    let mut grid = grid.clone();
    let mut accessible_rolls = get_accessible_rolls(&grid);
    let mut num_accessible_rolls = count_accessible_rolls(&accessible_rolls);
    let mut rolls_removed = 0;
    while num_accessible_rolls > 0 {
        rolls_removed += num_accessible_rolls;
        grid = grid - accessible_rolls;
        accessible_rolls = get_accessible_rolls(&grid);
        num_accessible_rolls = count_accessible_rolls(&accessible_rolls);
    }
    rolls_removed
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let grid = parse_grid(filename)?;
    println!("\tGrid shape: {:?}", grid.shape());

    println!("\tPart 1: {}", part1(&grid));
    println!("\tPart 2: {}", part2(&grid));
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
