use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
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

fn calc_new_position(dial_size: i32, position: i32, instruction: &Instruction) -> i32 {
    let mut new_position = position;
    match instruction.direction {
        'L' => {
            new_position = (position - instruction.turns) % dial_size;
        }
        'R' => {
            new_position = (position + instruction.turns) % dial_size;
        }
        _ => panic!("Invalid direction"),
    }
    if new_position < 0 {
        new_position += 100
    }
    new_position
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let dial_size = 100;
    let mut position = 50;
    let mut num_times_dial_zero = 0;
    for instruction in instructions {
        position = calc_new_position(dial_size, position, instruction);
        if position == 0 {
            num_times_dial_zero += 1;
        }
    }
    num_times_dial_zero
}

fn times_crosses_zero(dial_size: i32, position: i32, instruction: &Instruction) -> i32 {
    let mut num_times_dial_zero = instruction.turns / dial_size;
    let turns_modulo = instruction.turns % dial_size;
    if turns_modulo == 0 || position == 0{
        return num_times_dial_zero
    }
    match instruction.direction {
        'L' => {
            if turns_modulo >= position {
                num_times_dial_zero += 1;
            }
        }
        'R' => {
            if turns_modulo >= (dial_size - position) {
                num_times_dial_zero += 1;
            }
        }
        _ => panic!("Invalid direction"),
    }
    num_times_dial_zero
}

fn part2(instructions: &Vec<Instruction>) -> i32 {
    let dial_size = 100;
    let mut position = 50;
    let mut num_times_dial_zero = 0;
    for instruction in instructions {
        num_times_dial_zero += times_crosses_zero(dial_size, position, &instruction);
        position = calc_new_position(dial_size, position, instruction);
    }
    num_times_dial_zero
}

fn solve(filename: &Path) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let instructions = reader.lines().map(|line| parse_line(line.unwrap())).collect::<Vec<Instruction>>();

    println!("\tPart 1: {}", part1(&instructions));
    println!("\tPart 2: {}", part2(&instructions));
}

fn main() {
    println!("===Test===");
    solve(Path::new("./test"));
    println!();
    println!("===Input===");
    solve(Path::new("./input"));
}
