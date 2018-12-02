#!/usr/bin/env python3

import sys
from itertools import combinations
from collections import Counter

def keep_matching(word_a, word_b):
    return "".join([let_a for let_a, let_b in zip(word_a, word_b) if let_a == let_b])

def main(filename):
    with open(filename, 'r') as infile:
        lines = [line.strip() for line in infile.readlines()]

    num_twos = 0
    num_threes = 0
    for line in lines:
        letter_counts = Counter(line)
        counts = Counter([count for (letter, count) in letter_counts.most_common()])
        if counts[2] > 0:
            num_twos += 1
        if counts[3] > 0:
            num_threes += 1
    print("Part 1: {}".format(num_twos * num_threes))

    box_combos = combinations(lines, 2)
    for a, b in box_combos:
        matching = keep_matching(a, b)
        if len(matching) == (len(a) - 1):
            print(f"Part 2: {matching}")
            break

if len(sys.argv) > 1:
    main(sys.argv[1])
else:
    main('./input.txt')
