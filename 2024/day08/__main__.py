import os
import argparse

from collections import defaultdict
import itertools
from typing import Generator

from common.coords import Coordinates

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

class City:
    def __init__(self, dimensions, antennas):
        self.dimensions = dimensions
        self.antennas = antennas

    @staticmethod
    def from_file(input_file):
        antennas = defaultdict(list)
        for row_idx, line in enumerate(get_lines(input_file)):
            for col_idx, val in enumerate(line.strip()):
                if val != '.':
                    antennas[val].append(Coordinates(row_idx, col_idx))
        return City((row_idx+1, col_idx+1), antennas)

    def in_bounds(self, coords: Coordinates) -> bool:
        return coords.row >= 0 and coords.row < self.dimensions[0] and \
            coords.col >= 0 and coords.col < self.dimensions[1]

    def antinodes(self):
        antinodes = set()
        for antennas in self.antennas.values():
            for pairs in itertools.combinations(antennas, 2):
                diff = pairs[0] - pairs[1]
                for coords in (pairs[0] + diff, pairs[1] - diff):
                    if self.in_bounds(coords):
                        antinodes.add(coords)
        return antinodes

    def line_of_antinodes(self, antenna_pair) -> Generator[Coordinates, None, None]:
        diff = antenna_pair[0] - antenna_pair[1]
        for starting_pos, func in (
                (antenna_pair[0], lambda x: x + diff),
                (antenna_pair[1], lambda x: x - diff),
        ):
            curr_pos = starting_pos
            while self.in_bounds(curr_pos):
                yield curr_pos
                curr_pos = func(curr_pos)

    def resonant_antinodes(self):
        antinodes = set()
        for antennas in self.antennas.values():
            for pair in itertools.combinations(antennas, 2):
                for antinode in self.line_of_antinodes(pair):
                    antinodes.add(antinode)
        return antinodes

def part1(input_file):
    city = City.from_file(input_file)
    return len(city.antinodes())

def part2(input_file):
    city = City.from_file(input_file)
    return len(city.resonant_antinodes())

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
