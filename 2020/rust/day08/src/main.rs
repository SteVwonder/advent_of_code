#![feature(str_split_once)]

#[macro_use]
extern crate lazy_static;

use std::collections::HashSet;
use std::io::{self, Read};

use anyhow::{anyhow, Result};
use petgraph::dot::{Config, Dot};
use petgraph::prelude::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use regex::Regex;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Clone)]
enum Operation {
    Accum,
    Jump,
    Noop,
}

#[derive(Clone)]
struct Instruction {
    op: Operation,
    arg: i32,
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>,
    accumulator: i32,
    pc: usize,
}

impl Program {
    fn exec(&mut self) {
        let inst = self.instructions.get(self.pc).unwrap();
        match &inst.op {
            Operation::Accum => {
                self.accumulator += inst.arg;
                self.pc += 1
            }
            Operation::Jump => self.pc = ((self.pc as i32) + inst.arg) as usize,
            Operation::Noop => self.pc += 1,
        }
    }
}

fn parse_inst_from_line(input: &str) -> Instruction {
    let (op_str, arg_str) = input.split_once(" ").unwrap();
    let op = match op_str {
        "acc" => Operation::Accum,
        "jmp" => Operation::Jump,
        "nop" => Operation::Noop,
        _ => panic!("Invalid op input"),
    };
    let arg = arg_str.parse::<i32>().unwrap();

    Instruction { op, arg }
}

fn parse_program_from_str(input: &str) -> Program {
    let inst = input
        .lines()
        .map(|line| parse_inst_from_line(line))
        .collect();
    Program {
        instructions: inst,
        accumulator: 0,
        pc: 0,
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut program = parse_program_from_str(input);
    let mut pc_hist = HashSet::<usize>::new();
    while !pc_hist.contains(&program.pc) {
        pc_hist.insert(program.pc);
        program.exec();
    }
    Ok(program.accumulator)
}

fn runs_until_completion(program: &mut Program) -> bool {
    let mut pc_hist = HashSet::<usize>::new();
    while (!pc_hist.contains(&program.pc)) && (program.pc < program.instructions.len()) {
        pc_hist.insert(program.pc);
        program.exec();
    }

    program.pc == program.instructions.len()
}

fn part2(input: &str) -> Result<i32> {
    let orig_program = parse_program_from_str(input);
    for curr_inst_pos in 0..orig_program.instructions.len() {
        let mut curr_program = orig_program.clone();
        let mut target_inst = curr_program.instructions.get_mut(curr_inst_pos).unwrap();
        match target_inst.op {
            Operation::Accum => {}
            Operation::Jump => {
                target_inst.op = Operation::Noop;
            }
            Operation::Noop => {
                target_inst.op = Operation::Jump;
            }
        }
        if runs_until_completion(&mut curr_program) {
            return Ok(curr_program.accumulator);
        }
    }

    Err(anyhow!("Changing instructions didn't work"))
}
