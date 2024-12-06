import re
import argparse
import itertools
from collections import defaultdict
from tqdm import trange

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def ninety_degrees(direction):
    if direction == (-1, 0):
        return(0, 1)
    elif direction == (0, 1):
        return(1, 0)
    elif direction == (1, 0):
        return(0, -1)
    elif direction == (0, -1):
        return(-1, 0)
    else:
        print(direction)
        raise RuntimeError()

def get_intersection_idx(direction, pos):
    if direction in [(0,1), (0, -1)]:
        return pos[0]
    elif direction in [(-1, 0), (1, 0)]:
        return pos[1]
    else:
        raise RuntimeError()

class Layout:
    def __init__(self, obstacles, dimensions, guard_pos):
        self.obstacles = obstacles
        self.dimensions = dimensions
        self.guard_pos = guard_pos

        self.cells_visited = {self.guard_pos}
        self.virtual_walls = set()
        self.new_blockades = set()
        self.guard_direction = (-1, 0)
        self.cycle_detection = {(self.guard_pos, self.guard_direction)}


    @staticmethod
    def from_file(input_file):
        lines = get_lines(input_file)

        obstacles = set()
        num_lines = 0
        for row_idx, line in enumerate(lines):
            row = [x for x in line.strip()]
            num_cols = len(row)
            num_lines += 1
            for col_idx, element in enumerate(row):
                if element == "#":
                    obstacles.add((row_idx, col_idx))
                elif element == "^":
                    guard_pos = (row_idx, col_idx)

        # TODO: do loop variables exist after the for loop ends?
        dimensions = (num_lines, num_cols)
        return Layout(obstacles, dimensions, guard_pos)

    def rotate_guard(self):
        self.guard_direction = ninety_degrees(self.guard_direction)

    def walk(self) -> bool:
        while True:
            new_guard_pos = (self.guard_pos[0] + self.guard_direction[0],
                             self.guard_pos[1] + self.guard_direction[1])
            if new_guard_pos[0] >= self.dimensions[0] or \
               new_guard_pos[1] >= self.dimensions[1] or \
               new_guard_pos[0] < 0 or \
               new_guard_pos[1] < 0:
                return False
            elif new_guard_pos in self.obstacles:
                self.rotate_guard()
                continue

            pos_and_dir = (new_guard_pos, self.guard_direction)
            if pos_and_dir in self.cycle_detection:
                return True

            self.guard_pos = new_guard_pos
            self.cells_visited.add(new_guard_pos)
            self.cycle_detection.add(pos_and_dir)

def part1(input_file):
    layout = Layout.from_file(input_file)
    layout.walk()
    return len(layout.cells_visited)

def part2(input_file):
    layout = Layout.from_file(input_file)
    orig_guard_pos = layout.guard_pos
    layout.walk()
    num_warps = 0
    for row_idx in trange(layout.dimensions[0]):
        for col_idx in range(layout.dimensions[1]):
            pos = (row_idx, col_idx)
            if pos in layout.obstacles:
                continue
            if pos not in layout.cells_visited:
                continue
            new_obstacles = layout.obstacles.copy()
            new_obstacles.add(pos)
            new_layout = Layout(new_obstacles, layout.dimensions, orig_guard_pos)
            num_warps += new_layout.walk()
    return num_warps

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
