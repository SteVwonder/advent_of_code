use std::env;
use std::collections::{HashSet,HashMap};

use petgraph::algo::bellman_ford;
use petgraph::csr::Csr;
use petgraph::prelude::*;

type Graph = Csr<(), f32, Undirected, u32>;

enum Direction {
    South,
    East,
}

fn get_neighbors(node: &char) -> Vec<Direction> {
    match node {
        '.' => vec![], // no-op
        'J' => vec![], // would've already been handled by previous iteration
        '|' | '7' => vec![Direction::South],
        '-' | 'L' => vec![Direction::East],
        'F' | 'S' => vec![Direction::South, Direction::East],
        _ => unreachable!(),
    }
}

fn match_west_east(west: &char, east: &char) -> bool {
    match west {
        '-' | 'L' | 'F' | 'S' => (),
        _ => panic!("Invalid west"),
    };

    match east {
        'J' | '7' | '-' | 'S' => true,
        _ => false,
    }
}

fn match_north_south(north: &char, south: &char) -> bool {
    match north {
        '|' | '7' | 'F' | 'S' => (),
        _ => panic!("Invalid north"),
    };

    match south {
        'J' | '|' | 'L' | 'S' => true,
        _ => false,
    }
}

fn build_graph_from_lines(lines: Vec<Vec<char>>) -> (Graph, u32) {
    let num_rows = lines.iter().count();
    let num_cols = lines.iter().next().unwrap().len();
    let mut graph = Csr::with_nodes(num_rows * num_cols);
    let mut starting_node_idx = 0;
    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, node_char) in line.iter().enumerate() {
            let node_idx = ((row_idx * num_cols) + col_idx) as u32;
            if *node_char == 'S' {
                starting_node_idx = node_idx
            }
            let neighbors = get_neighbors(node_char);
            for direction in neighbors {
                let coords = match direction {
                    Direction::South => (row_idx + 1, col_idx),
                    Direction::East => (row_idx, col_idx + 1),
                };
                if let Some(neighbor_line) = lines.get(coords.0) {
                    if let Some(neighbor_char) = neighbor_line.get(coords.1) {
                        if match direction {
                            Direction::South => match_north_south(node_char, neighbor_char),
                            Direction::East => match_west_east(node_char, neighbor_char),
                        } {
                            let neighbor_idx = ((coords.0 * num_cols) + coords.1) as u32;
                            graph.add_edge(node_idx, neighbor_idx, 1.0);
                        }
                    }
                }
            }
        }
    }
    (graph, starting_node_idx)
}

fn filter_graph_to_loop(g: &Graph, start_idx: u32) -> Graph {
    let mut dfs = Dfs::new(&g, start_idx);
    let mut out_graph = Graph::with_nodes(g.node_count());

    while let Some(idx) = dfs.next(&g) {
        for edge in g.edges(idx) {
            if edge.source() < edge.target() {
                out_graph.add_edge(edge.source(), edge.target(), 1.0);
            }
        }
    }

    out_graph
}

fn part1(lines: Vec<Vec<char>>) -> u32 {
    let (full_graph, starting_idx) = build_graph_from_lines(lines);
    let g = filter_graph_to_loop(&full_graph, starting_idx);
    let mut dist = 1;
    let mut curr_edge_opt = g.edges(starting_idx).next();
    let mut predecessor = starting_idx;
    while curr_edge_opt.is_some() {
        let curr_node = curr_edge_opt.unwrap().target();
        if curr_node == starting_idx {
            break;
        }
        dist += 1;
        curr_edge_opt = g.edges(curr_node).filter(|edge| edge.target() != predecessor).next();
        predecessor = curr_node;
    }
    return dist / 2;
}

fn part2(lines: Vec<Vec<char>>) -> u32 {
    0
}

fn read_file_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.chars().collect())
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
    println!("Part1: {}", part1(lines.clone()));
    println!("Part2: {}", part2(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    type Example = Vec<Vec<char>>;

    fn get_examples() -> Vec<Example> {
        let mut examples: Vec<Vec<&'static str>> = vec![
            vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."],
            vec!["-L|F7", "7S-7|", "L|7||", "-L-J|", "L|-JF"],
            vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."],
            vec!["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"],
        ];
        examples
            .into_iter()
            .map(|example| {
                example
                    .into_iter()
                    .map(|line: &str| (*line).chars().collect::<Vec<char>>())
                    .collect::<Example>()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn filter_graph() {
        let examples = get_examples();
        for example in examples.iter().take(1) {
            println!("");
            println!("New Example!");
            let (g, start_idx) = build_graph_from_lines(example.clone());
            filter_graph_to_loop(&g, start_idx);
        }
    }

    #[test]
    fn part1_e2e() {
        let examples = get_examples();
        let answers: Vec<u32> = vec![4, 4, 8, 8];
        for (example, answer) in examples.iter().zip(answers.iter()) {
            assert_eq!(part1(example.clone()), *answer);
        }
    }
}
