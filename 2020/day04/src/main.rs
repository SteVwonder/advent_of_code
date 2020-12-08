#![feature(str_split_once)]

#[macro_use]
extern crate lazy_static;

use std::collections::{HashMap, HashSet};
use std::io::{self, Read};
use std::iter::FromIterator;

use anyhow::{anyhow, Result};
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

type Passport = HashMap<String, String>;

fn parse_from_str(input: &str) -> Passport {
    input
        .lines()
        .flat_map(|line| line.split(" "))
        .map(|x| {
            let (key, value) = x.split_once(":").unwrap();
            (String::from(key), String::from(value))
        })
        .collect()
}

fn validate(passport: &Passport) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in required_fields {
        if !passport.contains_key(field) {
            return false;
        }
    }
    return true;
}

fn bounds(passport: &Passport, key: &str, min: u32, max: u32) -> bool {
    let value = passport.get(key).unwrap().parse::<u32>().unwrap();
    (value >= min) && (value <= max)
}

fn validate_height(height_str: &String) -> bool {
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r"^([0-9]{2,3})(in|cm)$").unwrap();
    }
    match HEIGHT_RE.captures(height_str) {
        Some(captures) => {
            let height = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            match captures.get(2).unwrap().as_str() {
                "cm" => (height >= 150) && (height <= 193),
                "in" => (height >= 59) && (height <= 76),
                _ => false,
            }
        }
        None => false,
    }
}

fn validate2(passport: &Passport) -> bool {
    lazy_static! {
        static ref PASSPORT_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        static ref HAIR_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref VALID_EYES: HashSet<String> = HashSet::from_iter(
            vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .map(|x| String::from(*x))
        );
    }

    if !validate(passport) {
        return false;
    }
    bounds(passport, "byr", 1920, 2002)
        && bounds(passport, "iyr", 2010, 2020)
        && bounds(passport, "eyr", 2020, 2030)
        && passport.get("byr").unwrap().len() == 4
        && passport.get("iyr").unwrap().len() == 4
        && passport.get("eyr").unwrap().len() == 4
        && PASSPORT_RE.is_match(passport.get("pid").unwrap())
        && HAIR_RE.is_match(passport.get("hcl").unwrap())
        && validate_height(passport.get("hgt").unwrap())
        && VALID_EYES.contains(passport.get("ecl").unwrap())
}

fn get_from_input(input: &str) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut passport_lines = Vec::new();
    for line in input.lines() {
        if line == "" {
            passports.push(parse_from_str(passport_lines.join(" ").as_str()));
            passport_lines.clear();
        } else {
            passport_lines.push(line);
        }
    }
    passports.push(parse_from_str(passport_lines.join(" ").as_str()));
    passports
}

fn part1(input: &str) -> Result<u32> {
    let passports = get_from_input(input);
    Ok(passports.iter().map(|x| validate(x) as u32).sum())
}

fn part2(input: &str) -> Result<u32> {
    let passports = get_from_input(input);
    Ok(passports.iter().map(|x| validate2(x) as u32).sum())
}
