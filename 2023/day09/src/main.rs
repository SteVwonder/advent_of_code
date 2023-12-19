use std::{env, collections::HashMap};

use itertools::Itertools;

fn get_derivative(seq: &Vec<i32>) -> Vec<i32> {
    seq.iter().zip(seq.iter().skip(1)).map(|(x, y)| y - x).collect()
}

fn get_all_derivatives(seq: Vec<i32>) -> Vec<Vec<i32>> {
    let mut derivatives = Vec::<Vec<i32>>::new();
    derivatives.push(seq);
    while !derivatives.last().unwrap().iter().fold(true, |accum, x| {accum && (*x == 0)}) {
        let next_seq = get_derivative(&derivatives.last().unwrap());
        derivatives.push(next_seq);
    }
    derivatives
}

fn part1(lines: Vec<String>) {
    let mut score: i32 = 0;
    for line in lines.iter() {
        let seq = line.split(" ").filter_map(|x| x.parse::<i32>().ok()).collect();
        let derivatives = get_all_derivatives(seq);
        let last_values = derivatives.iter().map(|x| x.last().unwrap());
        score += last_values.sum::<i32>()
    }
    println!("Part1: {}", score);
}

fn part2(lines: Vec<String>) -> i32 {
    let mut score: i32 = 0;
    for line in lines.iter() {
        let seq = line.split(" ").filter_map(|x| x.parse::<i32>().ok()).collect();
        let derivatives = get_all_derivatives(seq);
        let first_values = derivatives.iter().map(|x| x.first().unwrap());
        score += first_values.rev().fold(0, |accum, x| x - accum);
    }
    println!("Part2: {}", score);
    score
}

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let lines = read_file_lines(filename);
    part1(lines.clone());
    part2(lines.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let lines = vec!("23 49 87 137 199 273".to_string());
        assert_eq!(part2(lines), 9);

        let lines = vec!("0 3 6 9 12 15".to_string());
        assert_eq!(part2(lines), -3);

        let lines = vec!("10  13  16  21  30  45".to_string());
        assert_eq!(part2(lines), 5);
    }

}

