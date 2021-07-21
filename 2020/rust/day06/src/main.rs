use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::FromIterator;

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

fn parse_part1_from_str(lines: &Vec<&str>) -> HashSet<char> {
    HashSet::from(
        lines
            .iter()
            .flat_map(|line| line.chars())
            .filter(|&ch| ch != ' ')
            .collect(),
    )
}

fn parse_part2_from_str(lines: &Vec<&str>) -> u32 {
    let num_people = lines.len();
    lines
        .iter()
        .flat_map(|line| line.chars())
        .filter(|&ch| ch != ' ')
        .sorted()
        .group_by(|x| x.clone())
        .into_iter()
        .map(|(key, group)| group.into_iter().fold(0, |accum, _x| accum + 1))
        .filter(|&x| x == num_people)
        .fold(0, |accum, _x| accum + 1)
}

// TODO: replace with itertools.batching
fn get_from_input<T>(input: &str, func: &dyn Fn(&Vec<&str>) -> T) -> Vec<T> {
    let mut responses = Vec::new();
    let mut response_lines = Vec::new();
    for line in input.lines() {
        if line == "" {
            responses.push(func(&response_lines));
            response_lines.clear();
        } else {
            response_lines.push(line);
        }
    }
    responses.push(func(&response_lines));
    responses
}

fn part1(input: &str) -> Result<usize> {
    let responses = get_from_input(input, &parse_part1_from_str);
    Ok(responses.iter().map(|resp| resp.len()).sum())
}

fn part2(input: &str) -> Result<u32> {
    let responses = get_from_input(input, &parse_part2_from_str);
    Ok(responses.iter().sum())
}
