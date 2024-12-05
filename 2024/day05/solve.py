import re
import argparse
import itertools
from collections import defaultdict

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def update_is_valid(rules, update):
    prev_seen = set()
    for page in update:
        applicable_rules = rules[page]
        rule_violations = applicable_rules.intersection(prev_seen)
        if len(rule_violations):
            return False
        prev_seen.add(page)
    return True

def part1(input_file):
    lines = get_lines(input_file)
    rules_lines = itertools.takewhile(lambda line: '|' in line, lines)

    rules = defaultdict(set)
    for line in rules_lines:
        a, b = line.split('|')
        rules[int(a)].add(int(b))

    updates = [
        [int(x) for x in line.split(',')]
        for line in lines
    ]

    middle_pages = 0
    for update in updates:
        if update_is_valid(rules, update):
            if len(update) % 2 == 0:
                raise RuntimeException()
            middle_pages += update[len(update) // 2]
    return middle_pages

def part2(input_file):
    pass

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test', action='store_true')
    args = parser.parse_args()

    input_file = 'input'
    if args.test:
        input_file = 'test'
    print(part1(input_file))
    print(part2(input_file))

if __name__ == "__main__":
    main()
