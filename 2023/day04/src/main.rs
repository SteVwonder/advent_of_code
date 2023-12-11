use std::collections::{HashMap, HashSet};
use std::env;

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn num_str_to_hashset(nums: &str) -> HashSet<u32> {
    return nums
        .trim()
        .split(" ")
        .filter_map(|x| x.trim().parse::<u32>().ok())
        .collect();
}

fn part1(lines: Vec<String>) {
    let mut score = 0;
    let base: i64 = 2;
    for line in lines.iter() {
        if let Some((_, numbers)) = line.split_once(": ") {
            if let Some((winning_nums_str, my_nums_str)) = numbers.trim().split_once("|") {
                let winning_nums = num_str_to_hashset(&winning_nums_str);
                let my_nums = num_str_to_hashset(&my_nums_str);

                let exponent = winning_nums.intersection(&my_nums).count();
                if exponent >= 1 {
                    score += base.pow(exponent as u32 - 1);
                }
            }
        }
    }
    println!("Part1: {}", score);
}

fn part2(lines: Vec<String>) {
    let mut scratchcard_counts = HashMap::<i32, i32>::new();
    let mut largest_card_id = 0;
    scratchcard_counts.insert(1, 1);
    for (idx, line) in lines.iter().enumerate() {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_nums_str, my_nums_str) = numbers.trim().split_once("|").unwrap();

        let card_id = (idx as i32) + 1;
        largest_card_id = largest_card_id.max(card_id);
        let winning_nums = num_str_to_hashset(&winning_nums_str);
        let my_nums = num_str_to_hashset(&my_nums_str);
        let common_nums = winning_nums.intersection(&my_nums).count() as i32;

        let num_scratchcards = scratchcard_counts.entry(card_id).or_insert(1).clone();
        for future_card_id in card_id + 1..card_id + common_nums + 1 {
            scratchcard_counts
                .entry(future_card_id)
                .and_modify(|x| *x += num_scratchcards)
                .or_insert(num_scratchcards + 1);
        }
    }
    let total_scratchcards: i32 = (1..largest_card_id + 1)
        .map(|x| scratchcard_counts.entry(x).or_default().clone())
        .sum();
    println!("Part2: {}", total_scratchcards);
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
