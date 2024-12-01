import re
import argparse
from collections import Counter

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def get_lists(input_file):
    lines = get_lines(input_file)

    As = []
    Bs = []
    for line in lines:
        a, b = re.split(' +', line)
        As.append(int(a))
        Bs.append(int(b))

    return As, Bs

def part1(input_file):
    As, Bs = get_lists(input_file)

    As.sort()
    Bs.sort()

    sum = 0
    for a, b in zip(As, Bs):
        sum += abs(a - b)
    return sum

def part2(input_file):
    As, Bs = get_lists(input_file)
    counts_in_B = Counter(Bs)
    sum = 0
    for a in As:
        sum += a * counts_in_B[a]
    return sum

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
