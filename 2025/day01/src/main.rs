use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    turns: i32,
}

// Creates an Instruction representing a direction (L or R) and a number of turns (i32) from lines that look like:
// L30
// R0
// L1000
// R42
fn parse_line(line: &str) -> Result<Instruction, Box<dyn Error>> {
    let direction = match line.chars().next().ok_or("Empty line")? {
        'L' => Direction::Left,
        'R' => Direction::Right,
        c => return Err(format!("Invalid direction: {}", c).into()),
    };
    let turns = line[1..].parse::<i32>()?;
    Ok(Instruction { direction, turns })
}

fn calc_new_position(dial_size: i32, position: i32, instruction: &Instruction) -> i32 {
    let offset = match instruction.direction {
        Direction::Left => -instruction.turns,
        Direction::Right => instruction.turns,
    };
    (position + offset).rem_euclid(dial_size)
}

fn part1(instructions: &[Instruction]) -> i32 {
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
    if turns_modulo == 0 || position == 0 {
        return num_times_dial_zero;
    }
    match instruction.direction {
        Direction::Left => {
            if turns_modulo >= position {
                num_times_dial_zero += 1;
            }
        }
        Direction::Right => {
            if turns_modulo >= (dial_size - position) {
                num_times_dial_zero += 1;
            }
        }
    }
    num_times_dial_zero
}

fn part2(instructions: &[Instruction]) -> i32 {
    let dial_size = 100;
    let mut position = 50;
    let mut num_times_dial_zero = 0;
    for instruction in instructions {
        num_times_dial_zero += times_crosses_zero(dial_size, position, instruction);
        position = calc_new_position(dial_size, position, instruction);
    }
    num_times_dial_zero
}

fn solve(filename: &Path) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let instructions = reader
        .lines()
        .map(|line| parse_line(&line?))
        .collect::<Result<Vec<Instruction>, _>>()?;

    println!("\tPart 1: {}", part1(&instructions));
    println!("\tPart 2: {}", part2(&instructions));
    Ok(())
}

fn main() {
    println!("===Test===");
    if let Err(e) = solve(Path::new("./test")) {
        eprintln!("Error solving test: {}", e);
    }
    println!();
    println!("===Input===");
    if let Err(e) = solve(Path::new("./input")) {
        eprintln!("Error solving input: {}", e);
    }
}
