use std::io::{self, Read};
use itertools::iproduct;
use anyhow::{Result,anyhow};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<(i32)> {
    let values: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    for (value_a, value_b) in iproduct!(values.clone(), values) {
        if value_a + value_b == 2020 {
            return Ok(value_a * value_b);
        }
    }
    Err(anyhow!("Failed to find values that sum to 2020"))
}

fn part2(input: &str) -> Result<(i32)> {
    let values: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    for (value_a, value_b, value_c) in iproduct!(values.clone(), values.clone(), values) {
        if value_a + value_b + value_c == 2020 {
            return Ok(value_a * value_b * value_c);
        }
    }
    Err(anyhow!("Failed to find values that sum to 2020"))
}
