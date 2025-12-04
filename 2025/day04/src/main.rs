use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use ndarray::Array2;
use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvFFTExt, ConvMode, PaddingMode};

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

fn part1(grid: &Array2<u8>) -> u64 {
    let kernel = array![
        [1, 1, 1],
        [1, 10, 1],
        [1, 1, 1],
    ];
    let convolution = grid.conv(&kernel, ConvMode::Same, PaddingMode::Zeros).unwrap();
    convolution.iter().filter(|x| **x >= 10 && **x < 14).count() as u64
}

fn part2(grid: &Array2<u8>) -> u64 {
    0
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
