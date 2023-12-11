use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Default,Debug)]
struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Default,Debug)]
struct Mapping {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

impl Mapping {
    fn map_src_dest(&self, val: u64) -> u64 {
        for range in self.ranges.iter() {
            let source_end = range.source_start + range.length;
            if val >= range.source_start && val < source_end {
                return range.destination_start + (val - range.source_start)
            }
        }
        return val
    }
}

fn part1(lines: Vec<String>) {
    let mut lines_iter = lines.iter();
    let seeds = lines_iter
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .split(" ")
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<u64>>()
        ;
    println!("Seeds: {:#?}", seeds);

    let mut maps = Vec::<Mapping>::new();
    let map_re = Regex::new(r"^([a-z]+)-to-([a-z]+) map:$").unwrap();
    for line in lines_iter.filter(|x| x.trim().len() > 0) {
        if let Some(caps) = map_re.captures(line) {
            let source = caps.get(1).unwrap().as_str().to_string();
            let destination = caps.get(2).unwrap().as_str().to_string();
            println!("Map line: {} to {}", source, destination);
            maps.push(Mapping {
                source,
                destination,
                ranges: Vec::new(),
            })
        } else {
            println!("Range line: {}", line);
            let mut vals = line.split(" ");
            if let Some(map) = maps.last_mut() {
                map.ranges.push(Range {
                    destination_start: vals.next().unwrap().parse().unwrap(),
                    source_start: vals.next().unwrap().parse().unwrap(),
                    length: vals.next().unwrap().parse().unwrap(),
                })
            } else {
                // TODO: error
            }
        }
    }
    println!("Maps: {:#?}", maps);
    let mut lowest_location = u64::MAX;
    for seed in seeds.iter() {
        let mut curr_value: u64 = seed.clone();
        for map in maps.iter() {
            let prev_value = curr_value.clone();
            curr_value = map.map_src_dest(curr_value);
            println!("{} {} mapped to {} {}", map.source, prev_value, map.destination, curr_value);
        }
        println!("Seed {} finally mapped to {}\n", seed, curr_value);
        lowest_location = u64::min(lowest_location, curr_value);
    }
    println!("Part1: {}", lowest_location);
}

fn part2(lines: Vec<String>) {}

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
