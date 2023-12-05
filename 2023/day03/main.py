import re
import argparse
from itertools import takewhile

symbols = {"+", "@", "-", "&", "$", "=", "#", "%", "/", "*"}

type Grid = list[list[str]]

def contains_symbol(grid: Grid, start_x, end_x, start_y, end_y) -> bool:
    for row in grid[start_y: end_y+1]:
        for char in row[start_x: end_x + 1]:
            if char in symbols:
                return True
    return False

def part1(grid: Grid) -> int:
    part_numbers = []
    for row_idx, row in enumerate(grid):
        # TODO: does the input support "wraparound"?
        for m in re.finditer(r'[0-9]+', "".join(row)):
            start_x = max(m.start() - 1, 0)
            # end is the end index (non-inclusive, so +1 is already added)
            end_x = min(m.end(), len(row) - 1)
            start_y = max(row_idx - 1, 0)
            end_y = min(row_idx + 1, len(grid) - 1)
            if contains_symbol(grid, start_x, end_x, start_y, end_y):
                part_numbers.append(int(m.group(0)))
    return sum(part_numbers)

def find_overlapping_matches(line, char_idx):
    overlapping_matches = []
    for m in re.finditer(r'[0-9]+', "".join(line)):
        if (m.start() <= char_idx + 1 and m.end() >= char_idx):
            overlapping_matches.append(m.group(0))
    return overlapping_matches

def find_adjacent_numbers(grid, row_idx, char_idx):
    left = "".join((takewhile(lambda x: x.isdigit(), reversed(grid[row_idx][0:char_idx]))))[::-1]
    right = "".join(takewhile(lambda x: x.isdigit(), grid[row_idx][char_idx+1:]))
    up, down = "", ""
    if row_idx > 0:
        up = find_overlapping_matches(grid[row_idx-1], char_idx)
    if row_idx < len(grid):
        down = find_overlapping_matches(grid[row_idx+1], char_idx)
    return [int(x) for x in [left, right, *up, *down] if len(x) > 0]

def part2(grid: Grid) -> int:
    score = 0
    for row_idx, row in enumerate(grid):
        for char_idx, char in enumerate(row):
            if char == "*": # a gear
                adjacent_numbers = find_adjacent_numbers(grid, row_idx, char_idx)
                if len(adjacent_numbers) == 2:
                    score += adjacent_numbers[0] * adjacent_numbers[1]
    return score

def main(args):
    grid = []
    with open(args.input_file) as fp:
        for line in fp:
            curr_row = [char for char in line]
            grid.append(curr_row)
    print(part1(grid))
    print(part2(grid))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    parser.add_argument('--part2', action='store_true')
    parser.add_argument('-v', '--verbose', action='store_true')
    args = parser.parse_args()
    main(args)
