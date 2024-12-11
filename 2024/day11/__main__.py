import os
import argparse

from itertools import chain

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def flatmap(func, iterable):
    return chain.from_iterable(map(func, iterable))

def expand_stone(stone):
    if stone == 0:
        return [1]

    str_stone = str(stone)
    if len(str_stone) % 2 == 0:
        halfway = len(str_stone) // 2
        return [int(x) for x in [str_stone[0:halfway], str_stone[halfway:]]]

    return [stone * 2024]

def expand_stones(input_file, times):
    lines = get_lines(input_file)
    stones = [int(x) for x in next(lines).rstrip().split(" ")]
    for _ in range(times):
        stones = flatmap(expand_stone, stones)
    return sum(1 for _ in stones)

def part1(input_file):
    return expand_stones(input_file, 25)

def part2(input_file):
    return expand_stones(input_file, 75)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test', action='store_true')
    args = parser.parse_args()

    input_file = 'input'
    if args.test:
        input_file = 'test'
    input_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), input_file)
    print(part1(input_file))
    print(part2(input_file))

if __name__ == "__main__":
    main()
