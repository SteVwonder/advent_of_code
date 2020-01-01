use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::convert::TryInto;

#[derive(Debug)]
struct State {
    memory: Vec<i32>,
    input: Vec<i32>,
    output: Vec<i32>,
    pc: usize,
}

#[derive(Debug)]
enum Operation {
    Add = 1,
    Mul = 2,
    Read = 3,
    Write = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
}

impl Operation {
    fn num_args(&self) -> usize {
        match self {
            Operation::Add         => 3,
            Operation::Mul         => 3,
            Operation::Read        => 1,
            Operation::Write       => 1,
            Operation::JumpIfTrue  => 2,
            Operation::JumpIfFalse => 2,
            Operation::LessThan    => 3,
            Operation::Equals      => 3,
        }
    }
    fn last_arg_write(&self) -> bool {
        match self {
            Operation::Add         => true,
            Operation::Mul         => true,
            Operation::Read        => true,
            Operation::Write       => false,
            Operation::JumpIfTrue  => false,
            Operation::JumpIfFalse => false,
            Operation::LessThan    => true,
            Operation::Equals      => true,
        }
    }
    fn increase_pc(&self, state: &mut State) {
        match self {
            Operation::JumpIfTrue  => {},
            Operation::JumpIfFalse => {},
            _                      => { state.pc += self.num_args() + 1; }
        };
    }
    fn run(&self, state: &mut State, args: Vec<i32>) {
        match self {
            Operation::Add   => {state.memory[args[2] as usize] = args[0] + args[1];},
            Operation::Mul   => {state.memory[args[2] as usize] = args[0] * args[1];},
            Operation::Read  => {state.memory[args[0] as usize] = state.input.pop().unwrap();},
            Operation::Write => {state.output.push(args[0]);},
            Operation::JumpIfTrue  => { state.pc = if args[0] != 0 {args[1] as usize}  else {state.pc + self.num_args() + 1} },
            Operation::JumpIfFalse => { state.pc = if args[0] == 0 {args[1] as usize}  else {state.pc + self.num_args() + 1} },
            Operation::LessThan    => { state.memory[args[2] as usize] = (args[0] < args[1]) as i32 },
            Operation::Equals      => { state.memory[args[2] as usize] = (args[0] == args[1]) as i32 },
        };
        self.increase_pc(state);
    }
}

impl From<u32> for Operation {
    fn from(opcode: u32) -> Operation {
        match opcode {
            1 => {Operation::Add}
            2 => {Operation::Mul},
            3 => {Operation::Read},
            4 => {Operation::Write},
            5 => {Operation::JumpIfTrue},
            6 => {Operation::JumpIfFalse},
            7 => {Operation::LessThan},
            8 => {Operation::Equals},
            _ => panic!("Invalid operation code {}", opcode),
        }
    }
}

/*
impl TryFrom<u32> for Operation {
    type Error = &'static str;

    fn try_from(opcode: u32) -> Result<Self, Self::Error> {
        match opcode {
            1 => {Ok(Operation::Add)}
            2 => {Ok(Operation::Mul)},
            3 => {Ok(Operation::Read)},
            4 => {Ok(Operation::Write)},
            _ => {Err("Invalid operation code {}", opcode)},
        }
    }
}
*/

fn set_instruction_output_bit(operation: &Operation, instruction: &mut u32) {
    if !operation.last_arg_write() {return;}
    let base_ten_mask: u32 = 10_u32.pow(operation.num_args() as u32 - 1);
    if (*instruction / (base_ten_mask)) % 10 == 0 {
        *instruction += base_ten_mask
    }
}

fn parse_args(in_args: &[i32], state: &State, instruction: u32) -> Vec<i32> {
    let num_args = in_args.len();
    let mut args: Vec<i32> = Vec::with_capacity(num_args);
    let mut instruction = instruction;
    for idx in 0..num_args {
        let value = in_args[idx as usize];
        args.push(if instruction % 10 == 0 {
            state.memory[value as usize]
        } else {
            value
        });
        instruction /= 10;
    };
    args
}

fn solve(state: &mut State, debug: bool) {
    loop {
        if state.pc >= state.memory.len() {
            panic!("Program Counter larger than program");
        }
        let mut instruction: u32 = match state.memory[state.pc].try_into() {
            Ok(x) => x,
            Err(_x) => panic!("Found negative instruction"),
        };
        let opcode = instruction % 100;
        if opcode == 99 {return;};
        let operation: Operation = match opcode.try_into() {
            Ok(x) => x,
            Err(x) => panic!(
                "Failed parsing opcode ({}) from instruction ({}): {}",
                opcode,
                instruction,
                x
            ),
        };
        instruction /= 100;
        set_instruction_output_bit(&operation, &mut instruction);
        let args = parse_args(
            &state.memory[
                state.pc + 1
                    ..
                    state.pc + 1 + operation.num_args()
            ],
            state,
            instruction,
        );
        if debug {
            println!("({:?})", operation);
            println!("\tArgs: {:?}, Raw Args: {:?}", args, &state.memory[(state.pc+1)..(state.pc+1+operation.num_args())]);
            print!("\t{:?}", state);
        }
        operation.run(state, args);
        if debug {
            println!(" -> {:?}", state);
        }
    }
}

fn parse_line(line: &String) -> Vec<i32> {
    line.split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn split_test_line(line: &String) -> Vec<String> {
    line.split(";").map(|x| x.to_string()).collect()
}

fn main() {
    let file = File::open(Path::new("./input")).unwrap();
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let program = parse_line(&line);
    let mut state_v1 = State {
        memory: program.clone(),
        input: vec![1],
        output: Vec::new(),
        pc: 0,
    };
    solve(&mut state_v1, false);
    for (idx, &output) in state_v1.output[0..state_v1.output.len() - 1].iter().enumerate() {
        if output != 0 {
            println!("Memory idx {} is {} (not 0)", idx, output);
        }
    }
    println!("Part 1: {}", state_v1.output[state_v1.output.len() - 1]);
    let mut state_v2 = State {
        memory: program.clone(),
        input: vec![5],
        output: Vec::new(),
        pc: 0,
    };
    solve(&mut state_v2, false);
    println!("Part 2: {}", state_v2.output[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_given_tests() {
        let file = File::open(Path::new("./test")).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let mut split_line = split_test_line(&line.unwrap());
            let input = split_line.remove(0);
            let expected = split_line.remove(0);
            assert!(split_line.len() == 0);
            let program = parse_line(&input);
            let mut state = State {
                memory: program,
                input: Vec::new(),
                output: Vec::new(),
                pc: 0,
            };
            solve(&mut state, true);
            assert_eq!(
                state.memory,
                parse_line(&expected),
            );
        }
    }

    #[test]
    fn day05_part2() {
        let file = File::open(Path::new("./test-output")).unwrap();
        let reader = BufReader::new(file);
        for (idx, line) in reader.lines().enumerate() {
            if idx > 0 {
                println!("");
            }
            println!("====Day05 Part2 Test {}=====", idx);
            let mut split_line = split_test_line(&line.unwrap());
            let program = parse_line(&split_line.remove(0));
            let input = parse_line(&split_line.remove(0));
            let output = parse_line(&split_line.remove(0));
            assert!(split_line.len() == 0);
            let mut state = State {
                memory: program,
                input: input,
                output: Vec::new(),
                pc: 0,
            };
            solve(&mut state, true);
            assert_eq!(
                state.output,
                output,
            );
        }
    }
}
