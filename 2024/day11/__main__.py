import os
import argparse

from functools import cache

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

@cache
def blink(stone, times) -> int:
    if times == 0:
        return 1
    else:
        if stone == 0:
            return blink(1, times-1)

        str_stone = str(stone)
        if len(str_stone) % 2 == 0:
            halfway = len(str_stone) // 2
            return blink(int(str_stone[0:halfway]), times-1) + blink(int(str_stone[halfway:]), times-1)

        return blink(stone * 2024, times-1)

def expand_file(input_file, times):
    lines = get_lines(input_file)
    stones = [int(x) for x in next(lines).rstrip().split(" ")]
    return sum(blink(stone, times) for stone in stones)

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
