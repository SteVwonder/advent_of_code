use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct Instruction {
    direction: char,
    turns: i32,
}

// Creates an Instruction representing a direction (L or R) and a number of turns (i32) from lines that look like:
// L30
// R0
// L1000
// R42
fn parse_line(line: String) -> Instruction {
    let direction = line.chars().next().unwrap();
    let turns = line.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
    Instruction { direction, turns }
}

fn solve(filename: &Path) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let instructions = reader.lines().map(|line| parse_line(line.unwrap())).collect::<Vec<Instruction>>();
    let dial_size = 100;
    let mut position = 50;
    let mut num_times_dial_zero = 0;
    for instruction in instructions {
        match instruction.direction {
            'L' => {
                position = (position - instruction.turns) % dial_size;
            }
            'R' => {
                position = (position + instruction.turns) % dial_size;
            }
            _ => panic!("Invalid direction"),
        }
        if position == 0 {
            num_times_dial_zero += 1;
        }
    }
    println!("\tPart 1: {}", num_times_dial_zero);
}

fn main() {
    println!("===Test===");
    solve(Path::new("./test"));
    println!();
    println!("===Input===");
    solve(Path::new("./input"));
}
