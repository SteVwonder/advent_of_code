use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn opcode_1(state: &mut Vec<u32>, arg1: usize, arg2: usize, arg3: usize) {
    state[arg3] = state[arg1] + state[arg2];
}
fn opcode_2(state: &mut Vec<u32>, arg1: usize, arg2: usize, arg3: usize) {
    state[arg3] = state[arg1] * state[arg2];
}

fn solve(state: &mut Vec<u32>, test: bool) {
    let mut pc: usize = 0;
    if test {
        println!("Starting state: {:?}", state);
        println!("({{opcode}}, {{arg1}}, {{arg2}}, {{arg3}}) - {{new state}}");
    }
    loop {
        if pc >= state.len() {
            panic!("Program Counter larger than program");
        }
        let opcode = state[pc];
        if opcode == 99 {return;}
        let arg1 = state[pc+1] as usize;
        let arg2 = state[pc+2] as usize;
        let arg3 = state[pc+3] as usize;
        pc += match opcode {
            1 => {
                opcode_1(state, arg1, arg2, arg3);
                4
            }
            2 => {
                opcode_2(state, arg1, arg2, arg3);
                4
            },
            _ => panic!("Invalid opcode {}", opcode),
        };
        if test {
            println!("({}, {}, {}, {}) - {:?}", opcode, arg1, arg2, arg3, state);
        }
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
    solve(&mut program_v1, false);
    println!("Part 1: {}", program_v1[0]);
    let upper_bound = (program_orig.len() - 1) as u32;
    let list: Vec<u32> = (0..upper_bound).collect();
    for (&a, &b) in list.iter().step_by(2).zip(list.iter().skip(1).step_by(2)) {
        let mut program = program_orig.clone();
        program[1] = a;
        program[2] = b;
        solve(&mut program, false);
        if program[0] == 19690720 {
            println!("Part 2: {}", 100 * a + b);
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
            let mut program = parse_line(input);
            solve(&mut program, true);
            assert_eq!(
                program,
                parse_line(expected),
            );
        }
    }
}
