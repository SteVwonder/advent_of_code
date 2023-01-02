#!/bin/env python3

import re
import argparse
import logging
from collections import defaultdict, namedtuple
from collections.abc import Mapping, Callable
from dataclasses import dataclass, field
from functools import cache
import itertools

from tqdm import tqdm
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


@cache
def rate_of_release(curr_pos: str, G: nx.DiGraph, open: frozenset[str], minutes_left: int) -> tuple[int, frozenset[str]]:
    best_ror = 0
    best_open = open
    if minutes_left <= 1:
        return best_ror, best_open

    for neighbor in nx.neighbors(G, curr_pos):
        curr_ror, curr_open = rate_of_release(neighbor, G, open, minutes_left - 1)
        if curr_ror > best_ror:
            best_ror = curr_ror
            best_open = curr_open

    curr_nodes_ror = nx.get_node_attributes(G, "rate")[curr_pos]
    if curr_nodes_ror == 0 or curr_pos in open:
        return best_ror, best_open

    curr_ror, curr_open = rate_of_release(curr_pos, G, open | frozenset([curr_pos]), minutes_left - 1)
    curr_ror += curr_nodes_ror * (minutes_left - 1)
    if curr_ror > best_ror:
        return curr_ror, curr_open
    return best_ror, best_open


def part1(lines, args) -> int:
    G = parse(lines)
    if args.plot:
        plot(G)
    ror, _ = rate_of_release("AA", G, frozenset(), 30)
    return ror


@cache
def dual_rate_of_release(curr_poses: tuple[str, str], G: nx.DiGraph, open: frozenset[str], minutes_left: int) -> tuple[int, frozenset[str]]:
    logging.debug(f'[{curr_poses} {open} {minutes_left}] DualRateOfRelase called')
    best_ror = 0
    best_open = open
    if minutes_left <= 1:
        logging.debug(f'[{curr_poses} {open} {minutes_left}] Too few minutes left ({minutes_left}). Returning: {best_ror} | {curr_poses}  | {best_open}')
        return best_ror, best_open

    # Both move
    movement_options = itertools.product(nx.neighbors(G, curr_poses[0]), nx.neighbors(G, curr_poses[1]))
    if minutes_left == part2_num_minutes:
        movement_options = tqdm(list(movement_options))
    for neighborA, neighborB in movement_options:
        new_poses = tuple(sorted([neighborA, neighborB]))
        curr_ror, curr_open = dual_rate_of_release(new_poses, G, open, minutes_left - 1)
        if curr_ror > best_ror:
            best_ror = curr_ror
            best_open = curr_open
            logging.debug(f'[{curr_poses} {open} {minutes_left}] Found a better option via both moving: {best_ror} | {curr_poses} -> {new_poses} | {open} -> {best_open} | {minutes_left - 1}')
        else:
            logging.debug(f'[{curr_poses} {open} {minutes_left}] Worse option for both moving: {curr_ror} | {curr_poses} -> {new_poses}  | {open} -> {curr_open} | {minutes_left - 1}')

    # One stays and unlocks, the other moves
    for idx_to_stay in [0, 1]:
        curr_nodes_ror = nx.get_node_attributes(G, "rate")[curr_poses[idx_to_stay]]
        if curr_nodes_ror == 0 or curr_poses[idx_to_stay] in open:
            continue

        idx_to_go = idx_to_stay - 1
        for neighbor in nx.neighbors(G, curr_poses[idx_to_go]):
            new_poses = list(curr_poses)
            new_poses[idx_to_go] = neighbor
            new_poses = tuple(sorted(new_poses))
            new_open = open | frozenset([curr_poses[idx_to_stay]])
            curr_ror, curr_open = dual_rate_of_release(new_poses, G, new_open, minutes_left - 1)
            curr_ror += curr_nodes_ror * (minutes_left - 1)
            if curr_ror > best_ror:
                best_ror = curr_ror
                best_open = curr_open
                logging.debug(f'[{curr_poses} {open} {minutes_left}] Found a better option via one move, one stay: {best_ror} | {curr_poses} -> {new_poses}  | {open} -> {best_open} | {minutes_left - 1}')
            else:
                logging.debug(f'[{curr_poses} {open} {minutes_left}] Worse option for one move, one stay: {curr_ror} | {curr_poses} -> {new_poses}  | {open} -> {curr_open} | {minutes_left - 1}')

    # Both stay
    if curr_poses[0] == curr_poses[1] or \
       curr_poses[0] in open or \
       curr_poses[1] in open:
        logging.debug(f'[{curr_poses} {open} {minutes_left}] Skipping checking both staying. Returning: {best_ror} | {curr_poses} -> {new_poses}  | {open} -> {best_open}')
        return best_ror, best_open
    curr_nodes_ror = [nx.get_node_attributes(G, "rate")[pos] for pos in curr_poses]
    if 0 in curr_nodes_ror:
        logging.debug(f'[{curr_poses} {open} {minutes_left}] One ror is 0. Skipping checking both staying. Returning: {best_ror} | {curr_poses} -> {new_poses}  | {open} -> {best_open}')
        return best_ror, best_open
    new_open = open | frozenset(curr_poses)
    curr_ror, curr_open = dual_rate_of_release(curr_poses, G, new_open, minutes_left - 1)
    curr_ror += (sum(curr_nodes_ror) * (minutes_left - 1))
    if curr_ror > best_ror:
        best_ror = curr_ror
        best_open = curr_open
        logging.debug(f'[{curr_poses} {open} {minutes_left}] Found a better option via both staying: {best_ror} | {curr_poses}  | {open} -> {best_open} | {minutes_left - 1}')
    else:
        logging.debug(f'[{curr_poses} {open} {minutes_left}] Worse option for both staying: {curr_ror} | {curr_poses} -> {new_poses}  | {open} -> {curr_open} | {minutes_left - 1}')

    return best_ror, best_open


part2_num_minutes = 10
def part2(lines, args) -> int:
    G = parse(lines)
    #ror, _ = dual_rate_of_release(("AA", "AA"), G, frozenset(), 26)
    ror, _ = dual_rate_of_release(("AA", "AA"), G, frozenset(), part2_num_minutes)
    return ror


def get_test_lines() -> list[str]:
    with open('test-input1', 'r') as infile:
        lines = [line.rstrip() for line in infile.readlines()]
    return lines

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
    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)

    if args.test_input > 0:
        with open('test-input'+str(args.test_input), 'r') as infile:
            lines = [line.rstrip() for line in infile.readlines()]
    else:
        from aocd import lines

    a = None
    #a = part1(lines, args)
    b = part2(lines, args)

    print(a)
    print(b)
    if not args.test_input:
        from aocd import submit
        #submit(a, part='a')
        #submit(b, part='b')

if __name__ == "__main__":
    main()
