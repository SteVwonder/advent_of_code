import os
import argparse

from collections import Counter
from itertools import chain
from tqdm import trange

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def flatmap(func, iterable):
    return chain.from_iterable(map(func, iterable))

def expand_stone_once(stone) -> Counter:
    if stone == 0:
        return Counter([1])

    str_stone = str(stone)
    if len(str_stone) % 2 == 0:
        halfway = len(str_stone) // 2
        return Counter([int(x) for x in [str_stone[0:halfway], str_stone[halfway:]]])

    return Counter([stone * 2024])

def multiply_counter(counter, multiple) -> Counter:
    for key in counter.keys():
        counter[key] *= multiple
    return counter

def expand_stones(stones) -> Counter:
    new_stones = Counter()
    for stone, count in stones.items():
        expansion = expand_stone_once(stone)
        new_stones += multiply_counter(expansion, count)
    return new_stones

def expand_file(input_file, times):
    lines = get_lines(input_file)
    stones = Counter([int(x) for x in next(lines).rstrip().split(" ")])
    for _ in trange(times):
        stones = expand_stones(stones)
    return stones.total()

def part1(input_file):
    return expand_file(input_file, 25)

def part2(input_file):
    return expand_file(input_file, 75)

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
