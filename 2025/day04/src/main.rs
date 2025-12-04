use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::LazyLock;

use ndarray::Array2;
use ndarray::prelude::*;
use ndarray_conv::{ConvExt, ConvMode, PaddingMode};

// Kernel: center cell contributes 10, neighbors contribute 1 each
static KERNEL: LazyLock<Array2<u8>> = LazyLock::new(|| {
    array![
        [1, 1, 1],
        [1, 10, 1],
        [1, 1, 1],
    ]
});

fn parse_grid(filename: &Path) -> Result<Array2<u8>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let rows: Vec<Vec<u8>> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|line| {
            line.chars()
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
    // Result >= 10 means cell is occupied (@)
    // Result < 14 means cell has 0-3 neighbors (accessible from edge)
    let convolution = grid
        .conv(&*KERNEL, ConvMode::Same, PaddingMode::Zeros)
        .unwrap();
    convolution.map(|&x| if x >= 10 && x < 14 { 1 } else { 0 })
}

fn count_accessible_rolls(accessible_rolls: &Array2<u8>) -> u64 {
    accessible_rolls.iter().copied().map(u64::from).sum()
}

fn part1(grid: &Array2<u8>) -> u64 {
    count_accessible_rolls(&get_accessible_rolls(grid))
}

fn part2(grid: &Array2<u8>) -> u64 {
    let mut grid = grid.clone();
    let mut rolls_removed = 0;

    loop {
        let accessible_rolls = get_accessible_rolls(&grid);
        let count = count_accessible_rolls(&accessible_rolls);

        if count == 0 {
            break;
        }

        rolls_removed += count;
        grid = grid - accessible_rolls;
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
