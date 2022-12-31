#!/bin/env python3

import re
import argparse
import logging
from collections import defaultdict, namedtuple
from collections.abc import Mapping, Callable
from dataclasses import dataclass, field

import networkx as nx
import matplotlib.pyplot as plt

line_re = re.compile(r'Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)')

def parse(lines) -> nx.DiGraph:
    G = nx.DiGraph()
    for line in lines:
        match = line_re.match(line)
        assert match is not None, line
        name = match.group(1)
        rate = int(match.group(2))
        connections = match.group(3).split(", ")
        G.add_node(name, rate=rate)
        for connection in connections:
            G.add_edge(name, connection)
    return G


def plot(G: nx.DiGraph):
    pos = nx.kamada_kawai_layout(G)
    rates = nx.get_node_attributes(G, 'rate')
    labels = {}
    for node, rate in rates.items():
        labels[node] = '{}\n{}'.format(node, rate)
    nx.draw_networkx(G, font_color='white', pos=pos, labels=labels, node_size=800)
    plt.show()


cache = {}
def cache_return(cache_tuple: tuple[str, frozenset, int], ret: tuple[int, frozenset[str]]) -> tuple[int, frozenset[str]]:
    cache[cache_tuple] = ret
    return ret


def rate_of_release(curr_pos: str, G: nx.DiGraph, open: frozenset[str], minutes_left: int) -> tuple[int, frozenset[str]]:
    best_ror = 0
    best_open = open
    if minutes_left <= 1:
        return best_ror, best_open

    cache_tuple = (curr_pos, open, minutes_left)
    if cache_tuple in cache:
        return cache[cache_tuple]

    for neighbor in nx.neighbors(G, curr_pos):
        curr_ror, curr_open = rate_of_release(neighbor, G, open, minutes_left - 1)
        if curr_ror > best_ror:
            best_ror = curr_ror
            best_open = curr_open

    curr_nodes_ror = nx.get_node_attributes(G, "rate")[curr_pos]
    if curr_nodes_ror == 0 or curr_pos in open:
        return cache_return(cache_tuple, (best_ror, best_open))

    curr_ror, curr_open = rate_of_release(curr_pos, G, open | frozenset([curr_pos]), minutes_left - 1)
    curr_ror += curr_nodes_ror * (minutes_left - 1)
    if curr_ror > best_ror:
        return cache_return(cache_tuple, (curr_ror, curr_open))
    return cache_return(cache_tuple, (best_ror, best_open))


def part1(lines, args) -> int:
    G = parse(lines)
    if args.plot:
        plot(G)
    ror, _ = rate_of_release("AA", G, frozenset(), 30)
    return ror


def part2(lines, args) -> int:
    pass


def test(lines):
    G = parse(lines)
    assert rate_of_release("CC", G, frozenset(), 2) == (2, frozenset(["CC"]))
    assert rate_of_release("CC", G, frozenset(), 1) == (0, frozenset())
    assert rate_of_release("CC", G, frozenset(), 0) == (0, frozenset())

    assert rate_of_release("EE", G, frozenset(), 3) == (20, frozenset(["DD"]))
    assert rate_of_release("EE", G, frozenset(), 4) == (40, frozenset(["DD"]))
    assert rate_of_release("EE", G, frozenset(), 5) == (63, frozenset(["DD", "EE"]))

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test-input', type=int, default=0)
    parser.add_argument(
        "--log-level",
        default=logging.INFO,
        type=lambda x: getattr(logging, x),
        help="Configure the logging level.",
    )
    parser.add_argument('--plot', action='store_true')
    parser.add_argument('--test', action='store_true')
    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)
    if args.test:
        with open('test-input1', 'r') as infile:
            lines = [line.rstrip() for line in infile.readlines()]
            test(lines)
            return

    if args.test_input > 0:
        with open('test-input'+str(args.test_input), 'r') as infile:
            lines = [line.rstrip() for line in infile.readlines()]
    else:
        from aocd import lines

    a = part1(lines, args)
    b = part2(lines, args)

    print(a)
    print(b)
    if not args.test_input:
        from aocd import submit
        submit(a, part='a')
        submit(b, part='b')

if __name__ == "__main__":
    main()
