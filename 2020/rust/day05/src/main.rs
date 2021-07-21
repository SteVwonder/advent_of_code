#[macro_use]
extern crate lazy_static;

use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

use anyhow::{anyhow, bail, Result};
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn get_dimension(
    boarding_pass: &str,
    re: &Regex,
    lower_bound_char: char,
    upper_bound_char: char,
    upper_bound: u32,
) -> Result<u32> {
    let partitioning_str = re
        .captures(boarding_pass)
        .ok_or(anyhow!("Invalid boarding pass - regex capture failed"))?
        .get(0)
        .unwrap()
        .as_str();
    let mut candidate_dims = (0, upper_bound);

    for partition_direction in partitioning_str.chars() {
        let distance = candidate_dims.1 - candidate_dims.0;
        let change = (distance as f32 / 2.0).ceil() as u32;
        if partition_direction == lower_bound_char {
            candidate_dims.0 += change;
        } else if partition_direction == upper_bound_char {
            candidate_dims.1 -= change;
        } else {
            bail!("Invalid boarding pass - invalid row char");
        }
        /*
        println!(
            "Direction: {}, Candidates: {:?}",
            partition_direction, candidate_dims
        );
         */
    }
    if (candidate_dims.1 - candidate_dims.0) == 0 {
        return Ok(candidate_dims.0);
    }
    bail!("Not enough chars in the boarding pass");
}

fn get_row(boarding_pass: &str) -> Result<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([BF]{7})").unwrap();
    }
    get_dimension(boarding_pass, &RE, 'B', 'F', 127)
}

fn get_col(boarding_pass: &str) -> Result<u32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([LR]{3})").unwrap();
    }
    get_dimension(boarding_pass, &RE, 'R', 'L', 7)
}

fn get_id(boarding_pass: &str) -> Result<u32> {
    let row = get_row(boarding_pass)?;
    let col = get_col(boarding_pass)?;
    Ok((row * 8) + col)
}

fn part1(input: &str) -> Result<u32> {
    let map: Result<Vec<u32>> = input.lines().map(|line| get_id(line)).collect();
    Ok(map?.iter().fold(0, |acc, x| max(acc, *x)))
}

fn part2(input: &str) -> Result<u32> {
    let mut my_seat = Vec::new();
    let res: Result<Vec<u32>> = input.lines().map(|line| get_id(line)).collect();
    let mut ids = res?;
    ids.sort_unstable();
    for window in ids.windows(2) {
        if (window[1] - window[0]) > 1 {
            for x in window[0]+1..window[1] {
                my_seat.push(x);
            }
        }
    }
    if my_seat.len() == 1 {
        return my_seat.pop().ok_or(anyhow!("Failed to pop item"));
    } else if my_seat.len() > 1 {
        bail!("Found too many missing seats");
    } else {
        bail!("No missing seats found");
    }
}
