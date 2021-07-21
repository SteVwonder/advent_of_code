use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};
use std::iter::FromIterator;

use anyhow::{anyhow, Result};
use itertools::iproduct;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let test_part1_res = test_part1(&input)?;
    println!("Test Part 1: {}", test_part1_res);
    println!("Test Part 2: {}", part2(&input, test_part1_res)?);
    let part1_res = part1(&input)?;
    println!("Part 1: {}", part1_res);
    println!("Part 2: {}", part2(&input, part1_res)?);
    Ok(())
}

fn prime_from_str(input: &str, num_to_fill: usize) -> VecDeque<u64> {
    let mut ret = VecDeque::with_capacity(num_to_fill);
    for line in input.lines().take(num_to_fill) {
        ret.push_front(line.parse::<u64>().unwrap());
    }
    ret
}

fn summable(circ_buf: &VecDeque<u64>, value: u64) -> bool {
    for (a, b) in iproduct!(circ_buf.iter(), circ_buf.iter()) {
        if a == b {
            continue;
        } else if a + b == value {
            return true;
        }
    }
    return false;
}

fn find_errant_value(input: &str, preamble_size: usize) -> Result<u64> {
    let mut circ_buf = prime_from_str(input, preamble_size);
    let remaining = input.lines().skip(preamble_size);
    for line in remaining {
        let value = line.parse::<u64>()?;
        if !summable(&circ_buf, value) {
            return Ok(value);
        }
        circ_buf.pop_back();
        circ_buf.push_front(value);
    }
    Err(anyhow!("Nothing summable"))
}

fn find_contig_sum(values: &Vec<u64>, target_sum: u64) -> Option<(usize, usize)> {
    for starting_idx in 0..values.len() {
        let mut curr_sum = 0;
        for (idx, value) in values.iter().skip(starting_idx).enumerate() {
            curr_sum += value;
            if curr_sum == target_sum {
                return Some((starting_idx, starting_idx + idx));
            } else if curr_sum > target_sum {
                break;
            }
        }
    }
    None
}

fn test_part1(input: &str) -> Result<u64> {
    find_errant_value(input, 5)
}

fn part1(input: &str) -> Result<u64> {
    find_errant_value(input, 25)
}

fn part2(input: &str, target: u64) -> Result<u64> {
    let values: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let (start, end) = find_contig_sum(&values, target).unwrap();
    let mut curr_min = std::u64::MAX;
    let mut curr_max = 0;
    for val in values.iter().skip(start).take(end-start) {
        curr_min = std::cmp::min(curr_min, *val);
        curr_max = std::cmp::max(curr_max, *val);
    }

    Ok(curr_min + curr_max)
}
