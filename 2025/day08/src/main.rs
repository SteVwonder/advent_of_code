use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

use itertools::Itertools;
use petgraph::Graph;
use petgraph::algo::{tarjan_scc, connected_components};
use petgraph::prelude::*;

#[derive(Debug,PartialEq,Eq,Hash,Clone)]
struct Junction {
    coords: [i64; 3],
}

impl fmt::Display for Junction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.coords[0], self.coords[1], self.coords[2])
    }
}

impl Junction {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { coords: [x, y, z] }
    }

    fn from_array(coords: [i64; 3]) -> Self {
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

fn parse_contents(contents: &str) -> Result<Vec<Junction>, Box<dyn Error>> {
    Ok(contents.lines().map(|line| {
        let coords: [i64; 3] = line.split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Expected exactly 3 coordinates");
        Junction::from_array(coords)
    }).collect())
}

fn part1(contents: &str, n: usize) -> i64 {
    let junctions = parse_contents(contents).unwrap();
    let distances: Vec<_> = junctions.iter().tuple_combinations()
        .map(|(a, b)| (a, b, a.distance_squared(b)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .take(n)
        .collect();

    let mut graph = Graph::new_undirected();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for junction in &junctions {
        let idx = graph.add_node(0);
        node_map.insert(junction.to_string(), idx);
    }

    for (a, b, _distance) in distances {
        let a_idx = node_map.get(&a.to_string()).unwrap();
        let b_idx = node_map.get(&b.to_string()).unwrap();
        graph.add_edge(*a_idx, *b_idx, 1);
    }

    let sccs = tarjan_scc(&graph);
    sccs.iter().map(|scc| scc.len()).sorted().rev().take(3).product::<usize>() as i64
}

fn part2(contents: &str, n: usize) -> i64 {
    let junctions = parse_contents(contents).unwrap();
    let distances: Vec<_> = junctions.iter().tuple_combinations()
        .map(|(a, b)| (a, b, a.distance_squared(b)))
        .sorted_by_key(|(_, _, distance)| *distance)
        .collect();

    let mut graph = Graph::new_undirected();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for junction in &junctions {
        let idx = graph.add_node(0);
        node_map.insert(junction.to_string(), idx);
    }

    for (idx, (a, b, _distance)) in distances.iter().enumerate() {
        let a_idx = node_map.get(&a.to_string()).unwrap();
        let b_idx = node_map.get(&b.to_string()).unwrap();
        graph.add_edge(*a_idx, *b_idx, 1);
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
