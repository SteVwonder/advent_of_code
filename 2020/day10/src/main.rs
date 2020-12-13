#[macro_use]
extern crate cached;

use cached::UnboundCache;
use std::io::{self, Read};

use anyhow::{Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let part1_res = part1(&input)?;
    println!("Part 1: {}", part1_res);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut values: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    values.push(0);
    values.sort();

    let mut diffs: Vec<u32> = values[..]
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect();
    diffs.sort();

    let num_ones = diffs.iter().filter(|&&x| x == 1).count();
    let num_threes = diffs.iter().filter(|&&x| x == 3).count() + 1;
    Ok((num_ones * num_threes) as u32)
}

cached_key! {
    CALC_WAYS: UnboundCache<String, u64> = UnboundCache::new();
    Key = { format!("{}{}", values.len(), curr) };
    fn calculate_ways(values: &[u32], curr: u32) -> u64 = {
        if values.len() == 1 {
            return 1;
        }

        let mut ret = 0;
        let mut mut_values = values;
        while mut_values.len() >= 2 && mut_values[0] - curr <= 3 {
            ret += calculate_ways(&mut_values[1..], mut_values[0]);
            mut_values = &mut_values[1..];
        }
        ret
    }
}

fn part2(input: &str) -> Result<u64> {
    let mut values: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    let last_adapter = values.iter().max().unwrap().clone();
    values.push(last_adapter + 3);
    values.sort();

    Ok(calculate_ways(&values[..], 0))
}
