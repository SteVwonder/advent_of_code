use anyhow::{anyhow, Result};
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

type TreeMap = Vec<Vec<bool>>;

fn parse_from_str(input: &str) -> Result<TreeMap> {
    Ok(input
        .lines()
        .map(|line| line.chars().map(|x| x == '#').collect())
        .collect())
}

fn check_collisions(map: &TreeMap, slope: (usize, usize)) -> u32 {
    let mut count = 0;
    let mut position = (0, 0);

    while position.0 < map.len() {
        if map[position.0][position.1] {
            count += 1;
        }

        position.0 += slope.0;
        position.1 = (position.1 + slope.1) % map[0].len();
    }

    count
}

fn part1(input: &str) -> Result<u32> {
    let map = parse_from_str(input)?;
    Ok(check_collisions(&map, (1, 3)))
}

fn part2(input: &str) -> Result<u64> {
    let map = parse_from_str(input)?;
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    Ok(slopes
        .iter()
        .map(|x| {
            check_collisions(&map, *x) as u64
        })
        .fold(1 as u64, |acc, x| acc * x))
}
