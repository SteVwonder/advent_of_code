use std::{env, collections::HashMap};
use num::integer::lcm;

type NodeIndex = String;

#[derive(Debug,PartialEq,Eq,Clone)]
struct Node {
    datum: NodeIndex,
    left: NodeIndex,
    right: NodeIndex,
}

#[derive(Debug,Default)]
struct Graph {
    nodes: HashMap<NodeIndex, Node>,
}

impl Graph {
    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.datum.clone(), node);
    }

    fn from_lines(lines: core::slice::Iter<String>) -> Graph {
        let mut graph = Graph::default();
        for line in lines {
            let (datum, edges_str) = line
                .split_once(" = ")
                .unwrap();
            let x: &[_] = &['(', ')'];
            let (left, right) = edges_str
                .trim_matches(x)
                .split_once(", ")
                .unwrap();
            graph.add_node(Node{
                datum: datum.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            });
        }
        return graph;
    }
}

fn get_steps_to_end(
    g: &Graph,
    dir_line: &String,
    starting_point: NodeIndex,
    is_end: fn(&NodeIndex) -> bool,
) -> u64 {
    let directions = dir_line.chars().cycle().enumerate();
    let mut curr_node = g.nodes.get(&starting_point).unwrap();
    for (idx, direction) in directions {
        if is_end(&curr_node.datum) {
            return idx as u64;
        }
        let next_index = match direction {
            'L' => &curr_node.left,
            'R' => &curr_node.right,
            _ => panic!("Invalid direction"),
        };
        curr_node = g.nodes.get(next_index).unwrap();
    }
    return 0;
}

fn part1(lines: Vec<String>) {
    let mut it = lines.iter();
    let dir_line = it.next().unwrap();
    it.next(); //empty line
    let g = Graph::from_lines(it);
    let score = get_steps_to_end(
        &g,
        &dir_line.to_string(),
        "AAA".to_string(),
        |x| {x == "ZZZ"}
    );
    println!("Part1: {}", score);
}

fn part2(lines: Vec<String>) {
    let mut it = lines.iter();
    let dir_line = it.next().unwrap().clone();
    it.next(); //empty line
    let g = Graph::from_lines(it);
    let score = g.nodes
        .keys()
        .filter(|x| {x.ends_with("A")})
        .map(|x| get_steps_to_end(&g, &dir_line, x.clone(), |y| {y.ends_with("Z")}))
        .fold(1, lcm)
        ;
    println!("Part2: {}", score);
}

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
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
