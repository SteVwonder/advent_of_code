//use std::collections::{HashMap, HashSet};
use std::env;

use regex::Regex;
use itertools::Itertools;

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Default,Debug)]
struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Default,Debug)]
struct Mapping {
    source: String,
    destination: String,
    ranges: Vec<MapRange>,
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

    fn map_dest_src(&self, val: u64) -> u64 {
        for range in self.ranges.iter() {
            let dest_end = range.destination_start + range.length;
            if val >= range.destination_start && val < dest_end {
                return range.source_start + (val - range.destination_start)
            }
        }
        return val
    }
}

#[derive(Default,Debug)]
struct Range {
    start: u64,
    length: u64,
}

#[derive(Default,Debug)]
struct Seeds {
    seeds: Vec<Range>,
}

impl IntoIterator for Seeds {
    type Item = u64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.seeds.iter().flat_map(|seed| {
            seed.start..seed.start+seed.length
        }).collect::<Vec<u64>>().into_iter()
    }
}

impl Seeds {
    fn for_part1(line: String) -> Seeds {
        let vals = line
            .split_once(":")
            .unwrap()
            .1
            .split(" ")
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let seeds = vals.iter().map(|x| {
            Range{
                start: *x,
                length: 1,
            }
        }).collect();
        Seeds{
            seeds
        }
    }

    fn for_part2(line: String) -> Seeds {
        let vals = line
            .split_once(":")
            .unwrap()
            .1
            .split(" ")
            .filter_map(|x| x.parse::<u64>().ok())
            .collect::<Vec<u64>>();
        let seeds = vals.iter().chunks(2).into_iter().map(
            |chunk| {
                let mut it = chunk.into_iter();
                Range{
                    start: it.next().unwrap().clone(),
                    length: it.next().unwrap().clone(),
                }
        }).collect();
        Seeds{
            seeds
        }
    }

    fn is_valid(&self, possible_seed: u64) -> bool {
        for seed in self.seeds.iter() {
            if possible_seed >= seed.start && possible_seed < seed.start + seed.length {
                return true
            }
        }
        return false
    }
}

fn maps_from_lines<I>(lines_iter: I) -> Vec<Mapping>
where
    I: Iterator<Item = String>,
{
    let mut maps = Vec::<Mapping>::new();
    let map_re = Regex::new(r"^([a-z]+)-to-([a-z]+) map:$").unwrap();
    for line in lines_iter.filter(|x| x.trim().len() > 0) {
        if let Some(caps) = map_re.captures(line.as_str()) {
            let source = caps.get(1).unwrap().as_str().to_string();
            let destination = caps.get(2).unwrap().as_str().to_string();
            maps.push(Mapping {
                source,
                destination,
                ranges: Vec::new(),
            })
        } else {
            let mut vals = line.split(" ");
            if let Some(map) = maps.last_mut() {
                map.ranges.push(MapRange {
                    destination_start: vals.next().unwrap().parse().unwrap(),
                    source_start: vals.next().unwrap().parse().unwrap(),
                    length: vals.next().unwrap().parse().unwrap(),
                })
            } else {
                // TODO: error
            }
        }
    }
    return maps;
}

fn part1(lines: Vec<String>) {
    let mut lines_iter = lines.into_iter();
    let seeds = Seeds::for_part1(lines_iter.next().unwrap());
    let maps = maps_from_lines(lines_iter);

    let mut lowest_location = u64::MAX;
    for seed in seeds.into_iter() {
        let mut curr_value: u64 = seed.clone();
        for map in maps.iter() {
            curr_value = map.map_src_dest(curr_value);
        }
        lowest_location = u64::min(lowest_location, curr_value);
    }
    println!("Part1: {}", lowest_location);
}

fn part2(lines: Vec<String>) {
    let mut lines_iter = lines.into_iter();
    let seeds = Seeds::for_part2(lines_iter.next().unwrap());
    let maps = maps_from_lines(lines_iter);

    let mut lowest_location = 0;
    for location in 0..u64::MAX {
        let mut curr_value = location;
        for map in maps.iter().rev() {
            curr_value = map.map_dest_src(curr_value);
        }
        if seeds.is_valid(curr_value) {
            lowest_location = location;
            break;
        }
    }

    println!("Part2: {}", lowest_location);
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
