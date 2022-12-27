#!/bin/env python3

import argparse
import logging
import re
from collections.abc import Mapping, Callable
from dataclasses import dataclass, field
import math
from operator import mul, add, floordiv, mod
from functools import reduce

import numpy as np
from matplotlib import pyplot as plt
import networkx as nx

s = 'abcdefghijklmnopqrstuvwxyz'


def traversable(heightA, heightB):
    a = s.find(heightA)
    b = s.find(heightB)
    return b - a <= 1


def get_value(lines, row, col) -> (str, str):
    real_val = lines[row][col]
    if real_val == "S":
        val = "a"
    elif real_val == "E":
        val = "z"
    else:
        val = real_val
    return val, real_val

def parse(lines, args) -> (nx.DiGraph, int, int):
    rows = len(lines)
    cols = len(lines[0])
    nodes = rows * cols
    graph = nx.DiGraph()

    start_idx = end_idx = 0

    positions = {}
    labels = {}
    graph.add_nodes_from(range(nodes))
    for idx in range(nodes):
        row = idx // cols
        col = idx % cols
        val, real_val = get_value(lines, row, col)
        if real_val == "S":
            start_idx = idx
        elif real_val == "E":
            end_idx = idx
        neighbors = [(row-1, col), (row+1, col), (row, col-1), (row, col+1)]
        neighbors = [n for n in neighbors if
                     ((n[0] >= 0) and (n[0] < rows) and
                      (n[1] >= 0) and (n[1] < cols))]
        positions[idx] = (col, rows - row)
        labels[idx] = val
        for n in neighbors:
            neighbor_idx = (n[0] * cols) + n[1]
            neighbor_val, _ = get_value(lines, n[0], n[1]) 
            if traversable(val, neighbor_val):
                logging.debug("({}, {}): {} -> {}".format(idx, neighbor_idx, val, neighbor_val))
                graph.add_edge(idx, neighbor_idx)

    if args.plot:
        node_colors = [0] * nodes
        path = nx.dijkstra_path(graph, start_idx, end_idx)
        for idx, node in enumerate(path):
            node_colors[node] = 0.5 + (idx / (len(path) * 2))
        nx.draw_networkx(graph, pos=positions, labels=labels, with_labels=True, font_color='white', node_color=node_colors)
        plt.tight_layout()
        plt.show()
    return graph, start_idx, end_idx


def part1(lines, args) -> int:
    graph, start, end = parse(lines, args)
    logging.debug(f"{start} -> {end}")
    path = nx.dijkstra_path(graph, start, end)
    logging.debug(path)
    return len(path) - 1


def part2(lines, args):
    graph, start, end = parse(lines, args)
    paths = nx.shortest_path(graph, target=end)
    min_dist = graph.number_of_nodes()
    cols = len(lines[0])
    for start_idx, path in paths.items():
        row = start_idx // cols
        col = start_idx % cols
        val, _ = get_value(lines, row, col)
        if val == 'a':
            logging.debug(f'Testing {start_idx: >4d} -> {end} ({len(path) - 1: >3d})')
            if (len(path) - 1) < min_dist:
                min_dist = (len(path) - 1)
    return min_dist


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test', type=int, default=0)
    parser.add_argument(
        "--log-level",
        default=logging.INFO,
        type=lambda x: getattr(logging, x),
        help="Configure the logging level.",
    )
    parser.add_argument('--plot', action='store_true')
    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)
    if args.test > 0:
        with open('test-input'+str(args.test), 'r') as infile:
            lines = [line.rstrip() for line in infile.readlines()]
    else:
        from aocd import lines

    a = part1(lines, args)
    b = part2(lines, args)

    print(a)
    print(b)
    if not args.test:
        from aocd import submit
        submit(a, part='a')
        submit(b, part='b')

if __name__ == "__main__":
    main()
