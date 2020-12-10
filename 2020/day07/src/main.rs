#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::io::{self, Read};

use anyhow::{anyhow, Result};
use petgraph::dot::{Config, Dot};
use petgraph::prelude::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Graph;
use regex::Regex;

type InternalGraph = Graph<String, u32>;

struct BagGraph {
    graph: InternalGraph,
    name_idx_map: HashMap<String, petgraph::prelude::NodeIndex>,
}

impl BagGraph {
    fn with_capacity(nodes: usize, edges: usize) -> BagGraph {
        BagGraph {
            graph: Graph::<String, u32>::with_capacity(nodes, edges),
            name_idx_map: HashMap::with_capacity(nodes),
        }
    }

    fn insert_or_get_idx(&mut self, name: &String) -> NodeIndex {
        match self.name_idx_map.get(name) {
            Some(idx) => *idx,
            None => {
                let idx = self.graph.add_node(name.clone());
                self.name_idx_map.insert(name.clone(), idx);
                idx
            }
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

type ParsedEdge = (String, String, u32);
fn parse_edges_from_line(line: &str) -> Vec<ParsedEdge> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([a-z ]+) bags contain (.*)\.").unwrap();
        static ref INNER_RE: Regex = Regex::new(r"([0-9]+) ([a-z ]+) bags?").unwrap();
    }

    let mut edges = Vec::new();
    let caps = RE.captures(line).unwrap();
    let outer_bag = caps.get(1).unwrap().as_str();
    let inner_bags = caps.get(2).unwrap().as_str();
    for split in inner_bags.split(',') {
        let inner_caps_res = INNER_RE.captures(split);
        if inner_caps_res.is_some() {
            let inner_caps = inner_caps_res.unwrap();
            let inner_bag_count = inner_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let inner_bag = inner_caps.get(2).unwrap().as_str();
            edges.push((
                outer_bag.to_string(),
                inner_bag.to_string(),
                inner_bag_count,
            ));
        } else if split.trim() == "no other bags" {
            continue;
        } else {
            panic!("Failed to parse inner bag: {}", split);
        }
    }

    edges
}

fn parse_graph_from_str(input: &str, reverse_edges: bool) -> BagGraph {
    let premapped_edges: Vec<ParsedEdge> = input
        .lines()
        .flat_map(|line| parse_edges_from_line(line))
        .collect();
    let mut bag_graph = BagGraph::with_capacity(premapped_edges.len() / 4, premapped_edges.len());
    for premapped_edge in premapped_edges {
        let source = bag_graph.insert_or_get_idx(&premapped_edge.0);
        let dest = bag_graph.insert_or_get_idx(&premapped_edge.1);
        if reverse_edges {
            bag_graph.graph.add_edge(dest, source, premapped_edge.2);
        } else {
            bag_graph.graph.add_edge(source, dest, premapped_edge.2);
        }
    }

    bag_graph
}

fn part1(input: &str) -> Result<usize> {
    let bag_graph = parse_graph_from_str(input, true);
    //println!("{:?}", Dot::with_config(&bag_graph.graph, &[]));
    let idx = bag_graph.name_idx_map.get("shiny gold").unwrap();
    let mut dfs = Dfs::new(&bag_graph.graph, *idx);
    let mut count = 0;
    while let Some(nx) = dfs.next(&bag_graph.graph) {
        count += 1;
    }
    Ok(count - 1)
}

fn part2(input: &str) -> Result<u32> {
    let bag_graph = parse_graph_from_str(input, false);
    fn dfs_recursive(node: NodeIndex, multiplier: u32, graph: &InternalGraph) -> u32 {
        let mut accum = multiplier;
        let mut walker = graph.neighbors(node).detach();
        while let Some((nxt_edge, nxt_node)) = walker.next(graph) {
            accum += dfs_recursive(
                nxt_node,
                multiplier * graph.edge_weight(nxt_edge).unwrap(),
                graph,
            );
        }
        accum
    }
    let idx = bag_graph.name_idx_map.get("shiny gold").unwrap();
    Ok(dfs_recursive(*idx, 1, &bag_graph.graph) - 1)
}
