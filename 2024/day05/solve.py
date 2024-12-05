import re
import argparse
import itertools
from collections import defaultdict

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def get_rules_and_updates(input_file):
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
    return rules, updates

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
    rules, updates = get_rules_and_updates(input_file)
    middle_pages = 0
    for update in updates:
        if update_is_valid(rules, update):
            if len(update) % 2 == 0:
                raise RuntimeException()
            middle_pages += update[len(update) // 2]
    return middle_pages

def fix_invalid_update(rules, update):
    pages_before_idx = set(update)
    idx = len(update) - 1
    while idx > 0:
        curr_page = update[idx]
        applicable_rules = rules[curr_page]
        rule_violations = applicable_rules.intersection(pages_before_idx)
        if len(rule_violations) > 0:
            # TODO: this is slow, but we could optimize if we wanted later
            new_idx = min((update.index(violation) for violation in rule_violations))
            update.pop(idx)
            update.insert(new_idx, curr_page)
        else:
            idx -= 1
            pages_before_idx.remove(curr_page)
    return update

def part2(input_file):
    rules, updates = get_rules_and_updates(input_file)
    middle_pages = 0
    for update in updates:
        if not update_is_valid(rules, update):
            update = fix_invalid_update(rules, update)
            if len(update) % 2 == 0:
                raise RuntimeException()
            middle_pages += update[len(update) // 2]
    return middle_pages

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
