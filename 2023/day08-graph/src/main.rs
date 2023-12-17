use std::env;
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use num::integer::lcm;

type NodeIndex = String;
type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

#[derive(Debug, Default)]
struct Node {
    datum: NodeIndex,
    left: Option<WeakNodeRef>,
    right: Option<WeakNodeRef>,
}

impl Node {
    fn new(datum: &NodeIndex) -> NodeRef {
        Rc::new(RefCell::new(Node {
            datum: datum.clone(),
            left: None,
            right: None,
        }))
    }
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<NodeIndex, NodeRef>,
}

impl Graph {
    fn add_node(&mut self, datum: NodeIndex) {
        let node = Node::new(&datum);
        self.nodes.insert(datum.to_string(), node);
    }

    fn add_left_edge(&self, datum: NodeIndex, left: NodeIndex) {
        let node_rc = self.nodes.get(&datum.to_string()).unwrap();
        let mut node = node_rc.borrow_mut();
        if datum == left {
            node.left = Some(Rc::downgrade(node_rc));
        } else {
            let left_node = self.nodes.get(&left).unwrap();
            node.left = Some(Rc::downgrade(left_node));
        }
    }

    fn add_right_edge(&self, datum: NodeIndex, right: NodeIndex) {
        let node_rc = self.nodes.get(&datum.to_string()).unwrap();
        let mut node = node_rc.borrow_mut();
        if datum == right {
            node.right = Some(Rc::downgrade(node_rc));
        } else {
            let right_node = self.nodes.get(&right).unwrap();
            node.right = Some(Rc::downgrade(right_node));
        }
    }

    fn from_lines(lines: VecDeque<String>) -> Graph {
        let mut graph = Graph {
            nodes: HashMap::<NodeIndex, NodeRef>::new(),
        };

        for line in lines.iter() {
            let (datum, _) = line.split_once(" = ").unwrap();
            graph.add_node(datum.to_string());
        }
        for line in lines.iter() {
            let (datum, edges_str) = line.split_once(" = ").unwrap();
            let x: &[_] = &['(', ')'];
            let (left_str, right_str) = edges_str.trim_matches(x).split_once(", ").unwrap();
            graph.add_left_edge(datum.to_string(), left_str.to_string());
            graph.add_right_edge(datum.to_string(), right_str.to_string());
        }
        graph
    }
}

fn get_steps_to_end(
    g: &Graph,
    dir_line: &String,
    starting_point: NodeIndex,
    is_end: fn(&NodeIndex) -> bool,
) -> u64 {
    let directions = dir_line.chars().cycle().enumerate();
    let mut curr_node_rc: NodeRef = g.nodes.get(&starting_point).unwrap().clone();
    for (idx, direction) in directions {
        let cloned_rc = curr_node_rc.clone();
        let borrowed = cloned_rc.borrow();
        if is_end(&borrowed.datum) {
            return idx as u64;
        }
        let next_node = match direction {
            'L' => borrowed.left.clone(),
            'R' => borrowed.right.clone(),
            _ => panic!("Invalid direction"),
        };
        curr_node_rc = next_node.unwrap().upgrade().unwrap();
    }
    return 0;
}

fn part1(mut lines: VecDeque<String>) {
    let dir_line = lines.pop_front().unwrap(); // directions
    lines.pop_front(); // empty line

    let g = Graph::from_lines(lines);
    let score = get_steps_to_end(&g, &dir_line.to_string(), "AAA".to_string(), |x| x == "ZZZ");
    println!("Part1: {}", score);
}

fn part2(mut lines: VecDeque<String>) {
    let dir_line = lines.pop_front().unwrap(); // directions
    lines.pop_front(); // empty line

    let g = Graph::from_lines(lines);
    let score = g.nodes
        .keys()
        .filter(|x| {x.ends_with("A")})
        .map(|x| get_steps_to_end(&g, &dir_line, x.clone(), |y| {y.ends_with("Z")}))
        .fold(1, lcm)
        ;
    println!("Part2: {}", score);
}

fn read_file_lines(filename: &str) -> VecDeque<String> {
    let mut result = VecDeque::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push_back(line.to_string())
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
    part2(lines);
}
