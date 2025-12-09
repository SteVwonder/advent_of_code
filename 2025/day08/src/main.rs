use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use petgraph::Graph;
use petgraph::algo::{tarjan_scc, connected_components};
use petgraph::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Junction {
    coords: [i64; 3],
}

impl Junction {
    fn new(coords: [i64; 3]) -> Self {
        Self { coords }
    }

    // supposedly doing this without sqrt lets it auto-vectorize, and the
    // absolutely correct distance isn't necessary since we are just comparing
    // distances
    fn distance_squared(&self, other: &Junction) -> i64 {
        self.coords.iter()
            .zip(other.coords.iter())
            .map(|(x, y)| {
                let d = x - y;
                d * d
            })
            .sum()
    }
}

fn parse_contents(contents: &str) -> Vec<Junction> {
    contents.lines().map(|line| {
        let coords: [i64; 3] = line.split(',')
            .map(|x| x.parse::<i64>().expect("invalid number"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Expected exactly 3 coordinates");
        Junction::new(coords)
    }).collect()
}

fn build_graph(junctions: &[Junction]) -> (Graph<(), (), Undirected>, HashMap<&Junction, NodeIndex>) {
    let mut graph = Graph::new_undirected();
    let node_map: HashMap<&Junction, NodeIndex> = junctions
        .iter()
        .map(|j| (j, graph.add_node(())))
        .collect();
    (graph, node_map)
}

fn part1(contents: &str, n: usize) -> i64 {
    let junctions = parse_contents(contents);
    let distances: Vec<_> = junctions.iter().tuple_combinations()
        .map(|(a, b)| (a, b, a.distance_squared(b)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .take(n)
        .collect();

    let (mut graph, node_map) = build_graph(&junctions);

    for (a, b, _distance) in distances {
        let a_idx = node_map[a];
        let b_idx = node_map[b];
        graph.add_edge(a_idx, b_idx, ());
    }

    let sccs = tarjan_scc(&graph);
    sccs.iter().map(|scc| scc.len()).sorted().rev().take(3).product::<usize>() as i64
}

fn part2(contents: &str, n: usize) -> i64 {
    let junctions = parse_contents(contents);
    let distances: Vec<_> = junctions.iter().tuple_combinations()
        .map(|(a, b)| (a, b, a.distance_squared(b)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .collect();

    let (mut graph, node_map) = build_graph(&junctions);

    for (idx, (a, b, _distance)) in distances.iter().enumerate() {
        let a_idx = node_map[a];
        let b_idx = node_map[b];
        graph.add_edge(a_idx, b_idx, ());
        if idx > n && connected_components(&graph) == 1 {
            return a.coords[0] * b.coords[0];
        }
    }
    0
}

fn solve(filename: &Path, n: usize) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    println!("\tPart 1: {}", part1(&contents, n));
    println!("\tPart 2: {}", part2(&contents, n));
    Ok(())
}

fn main() {
    println!("===Test===");
    if let Err(e) = solve(Path::new("./test"), 10) {
        eprintln!("Error solving test: {}", e);
    }
    println!();
    println!("===Input===");
    if let Err(e) = solve(Path::new("./input"), 1000) {
        eprintln!("Error solving input: {}", e);
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
}
