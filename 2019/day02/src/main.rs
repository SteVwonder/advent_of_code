use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;

fn solve(mut state: Vec<u32>, test: bool) -> Vec<u32> {
    let mut pc: usize = 0;
    if test {
        println!("({{opcode}}, {{arg1}}, {{arg2}}, {{arg3}}) - {{state}}");
    }
    loop {
        if pc >= state.len() {
            panic!("Program Counter larger than program");
        }
        let opcode = state[pc];
        if opcode == 99 {return state;}
        let arg1 = state[pc+1] as usize;
        let arg2 = state[pc+2] as usize;
        let arg3 = state[pc+3] as usize;
        let value = match opcode {
            1 => state[arg1] + state[arg2],
            2 => state[arg1] * state[arg2],
            _ => panic!("Invalid opcode {}", opcode),
        };
        if test {
            println!("({}, {}, {}, {}, {}) - {:?}", opcode, value, arg1, arg2, arg3, state);
        }
        state[arg3] = value;
        pc += 4
    }
}

fn parse_line(line: String) -> Vec<u32> {
    line.split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

fn split_test_line(line: String) -> (String, String) {
    let mut split: Vec<String> = line.split(";").map(|x| x.to_string()).collect();
    assert_eq!(split.len(), 2);
    (split.remove(0), split.remove(0))
}

fn main() {
    let file = File::open(Path::new("./input")).unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let program_orig = parse_line(line);
    let mut program_v1 = program_orig.clone();
    // AOC provides these magic numbers for Part 1
    program_v1[1] = 12;
    program_v1[2] = 2;
    println!("Part 1: {}", solve(program_v1, false)[0]);
    let upper_bound = (program_orig.len() - 1) as u32;
    for combination in (0..upper_bound).combinations_with_replacement(2) {
        if let [a, b] = combination[..] {
            let mut program = program_orig.clone();
            program[1] = a;
            program[2] = b;
            if solve(program, false)[0] == 19690720 {
                println!("Part 2: {}", 100 * a + b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_given_tests() {
        let file = File::open(Path::new("./test")).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let (input, expected) = split_test_line(line.unwrap());
            assert_eq!(
                solve(parse_line(input), true),
                parse_line(expected),
            );
        }
    }
}
