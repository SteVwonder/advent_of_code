import re
import argparse
import itertools
from collections import defaultdict

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

class Layout:
    def __init__(self, input_file):
        lines = get_lines(input_file)

        self.obstacles = set()
        num_lines = 0
        for row_idx, line in enumerate(lines):
            row = [x for x in line.strip()]
            num_cols = len(row)
            num_lines += 1
            for col_idx, element in enumerate(row):
                if element == "#":
                    self.obstacles.add((row_idx, col_idx))
                elif element == "^":
                    self.guard_pos = (row_idx, col_idx)
                    self.guard_direction = (-1, 0)

        # TODO: do loop variables exist after the for loop ends?
        self.dimensions = (num_lines, num_cols)
        self.cells_visited = {self.guard_pos}

    def rotate_guard(self):
        if self.guard_direction == (-1, 0):
            self.guard_direction = (0, 1)
        elif self.guard_direction == (0, 1):
            self.guard_direction = (1, 0)
        elif self.guard_direction == (1, 0):
            self.guard_direction = (0, -1)
        elif self.guard_direction == (0, -1):
            self.guard_direction = (-1, 0)
        else:
            print(self.guard_direction)
            raise RuntimeError()

    def move_guard(self) -> bool:
        while True:
            new_guard_pos = (self.guard_pos[0] + self.guard_direction[0],
                             self.guard_pos[1] + self.guard_direction[1])
            if new_guard_pos[0] >= self.dimensions[0] or \
               new_guard_pos[1] >= self.dimensions[1] or \
               new_guard_pos[0] < 0 or \
               new_guard_pos[1] < 0:
                return True
            elif new_guard_pos in self.obstacles:
                return False
            self.guard_pos = new_guard_pos
            self.cells_visited.add(new_guard_pos)

def part1(input_file):
    layout = Layout(input_file)
    while not layout.move_guard():
        layout.rotate_guard()
    return len(layout.cells_visited)

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
