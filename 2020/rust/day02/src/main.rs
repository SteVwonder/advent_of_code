use anyhow::{anyhow, Result};
use regex::Regex;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

struct Policy {
    min: u32,
    max: u32,
    letter: char,
}

fn parse(input: &str) -> Result<Vec<(Policy, String)>> {
    let parse_regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)")?;

    Ok(input
        .lines()
        .map(|x| {
            let caps = parse_regex.captures(x).unwrap();
            (
                Policy {
                    min: caps.get(1).unwrap().as_str().parse().unwrap(),
                    max: caps.get(2).unwrap().as_str().parse().unwrap(),
                    letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                },
                String::from(caps.get(4).unwrap().as_str()),
            )
        })
        .collect())
}

fn count_matching_chars(input: &String, chr: char) -> u32 {
    input
        .chars()
        .map(|x| (x == chr) as u32)
        .fold(0, |acc, x| acc + x)
}

fn part1(input: &str) -> Result<u32> {
    let values = parse(input)?;
    Ok(values
        .iter()
        .map(|(policy, password)| {
            let count = count_matching_chars(password, policy.letter);
            ((count >= policy.min) && (count <= policy.max)) as u32
        })
        .fold(0, |acc, x| acc + x))
}

fn part2(input: &str) -> Result<u32> {
    let values = parse(input)?;
    Ok(values
        .iter()
       .map(|(policy, password)| {
           let chars: Vec<char> = password.chars().collect();
            ((chars[(policy.min - 1) as usize] == policy.letter)
                ^ (chars[(policy.max - 1) as usize] == policy.letter)) as u32
        })
        .fold(0, |acc, x| acc + x))
}
