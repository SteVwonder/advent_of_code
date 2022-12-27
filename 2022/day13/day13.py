#!/bin/env python3

import argparse
import logging
from itertools import islice
from functools import cmp_to_key
from collections.abc import Mapping, Callable

from lark import Lark, Transformer


def compare(list_a, list_b) -> bool:
    assert type(list_a) == list and type(list_b) == list
    for a, b in zip(list_a, list_b):
        if type(a) == int and type(b) == int:
            if a < b:
                return -1
            elif b < a:
                return 1
        else:
            if type(a) == int:
                a = [a]
            elif type(b) == int:
                b = [b]
            res = compare(a, b)
            if res is not None:
                return res
    if len(list_a) < len(list_b):
        return -1
    if len(list_a) > len(list_b):
        return 1
    return None


def part1(lines, args) -> int:
    lines = [line for line in lines if line != ""]
    sum = 0
    for idx, (lineA, lineB) in enumerate(zip(
            islice(lines, 0, len(lines), 2),
            islice(lines, 1, len(lines), 2)
    ), 1):
        a = parse_line(lineA)
        b = parse_line(lineB)
        if compare(a, b) < 0:
            sum += idx
    return sum


def part2(lines, args):
    lists = [parse_line(line) for line in lines if line != ""]
    divider_packets = [
        [[2]],
        [[6]],
    ]
    lists.extend(divider_packets)
    lists = sorted(lists, key=cmp_to_key(compare))
    for list in lists:
        print(list)
    return (lists.index(divider_packets[0]) + 1) * \
        (lists.index(divider_packets[1]) + 1)


class ConvertToIntList(Transformer):
    INTEGER = int

    def start(self, args):
        return args[0]

    def list(self, args):
        return args

l = Lark('''start: list
list: "[" ( _element  "," )* _element? "]"
_element: (INTEGER | list)
INTEGER: /[0-9]+/
''')
def parse_line(line):
    parsed = l.parse(line)
    return ConvertToIntList().transform(parsed)


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
