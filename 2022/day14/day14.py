#!/bin/env python3

import argparse
import logging
from itertools import pairwise, zip_longest
from functools import cmp_to_key
from collections.abc import Mapping, Callable
from dataclasses import dataclass, field

from matplotlib import pyplot as plt
from matplotlib.animation import FuncAnimation
from matplotlib.ticker import MultipleLocator
import numpy as np
import numpy.typing as npt

def expand_rock_line(line) -> list[tuple[int, int]]:
    rock_line = []
    corners = [[int(coord) for coord in corner.split(",")] for corner in line.split(" -> ")]
    assert len(corners) >= 2
    for cornerA, cornerB in pairwise(corners):
        if cornerA[0] == cornerB[0]:
            start = min(cornerA[1], cornerB[1])
            end = max(cornerA[1], cornerB[1])
            fill = cornerA[0]
            new_rocks = zip_longest([], range(start, end+1), fillvalue=fill)
        elif cornerA[1] == cornerB[1]:
            start = min(cornerA[0], cornerB[0])
            end = max(cornerA[0], cornerB[0])
            fill = cornerA[1]
            new_rocks = zip_longest(range(start, end+1), [], fillvalue=fill)
        else:
            raise RuntimeError(f"Corners don't align: {cornerA}, {cornerB}")
        rock_line.extend(new_rocks)
    return rock_line

class Simulation:
    def __init__(self, rock_coords_set, part2=False):
        assert len(rock_coords_set) > 0
        self.rock_coords_set = rock_coords_set
        self.frozen_sand_set = set()
        self.all_frozen_set = rock_coords_set.copy()
        self.sand_origin = (500, 0)
        self.mobile_sand = self.sand_origin
        self.fig, self.ax = plt.subplots()
        self.completed = False
        self.ani = None
        y_coords = [coord[1] for coord in self.rock_coords_set]
        self.lowest_rock = max(y_coords)
        self.part2 = part2
        if part2:
            # For visualization
            x_coords = [coord[0] for coord in self.rock_coords_set]
            start = min(x_coords) - 5
            end = max(x_coords) + 5
            for x in range(start, end):
                self.rock_coords_set.add((x, self.lowest_rock + 2))

    @staticmethod
    def from_lines(lines, part2=False):
        rock_coords_set = set()
        for line in lines:
            rock_coords_set = rock_coords_set | set(expand_rock_line(line))
        return Simulation(rock_coords_set, part2)

    def going_into_abyss(self, sand):
        return sand[1] >= self.lowest_rock

    def hit_floor(self, sand):
        return self.part2 and sand[1] == self.lowest_rock + 1

    def move_sand(self) -> (tuple[int, int], bool):
        sand = self.mobile_sand
        frozen = self.all_frozen_set
        down = (sand[0], sand[1] + 1)
        if down not in frozen and not self.hit_floor(sand):
            return down, False
        left = (sand[0] - 1, sand[1] + 1)
        if left not in frozen and not self.hit_floor(sand):
            return left, False
        right = (sand[0] + 1, sand[1] + 1)
        if right not in frozen and not self.hit_floor(sand):
            return right, False
        return sand, True

    def init_plot(self):
        x_coords = [coord[0] for coord in self.rock_coords_set]
        self.ax.set_xlim(min(x_coords) - 1, max(x_coords) + 1)
        y_coords = [coord[1] for coord in self.rock_coords_set]
        self.ax.set_ylim(-1, max(y_coords) + 1)
        self.ax.invert_yaxis()
        for coords in self.rock_coords_set:
            self.ax.text(coords[0], coords[1], "#", animated=False)
        self.ax.xaxis.set_major_locator(MultipleLocator(5))
        self.ax.xaxis.set_minor_locator(MultipleLocator(1))
        self.ax.grid(visible=True, which='both', axis='both', dashes=(4, 2), dash_joinstyle='round')
        self.mobile_sand_artist = self.ax.scatter([self.mobile_sand[0]], [self.mobile_sand[1]], animated=True)
        self.frozen_sand_artist = self.ax.scatter([], [], animated=True)
        return self.mobile_sand_artist, self.frozen_sand_artist

    def complete(self):
        self.completed = True
        if self.ani:
            self.ani.pause()

    def update(self, i):
        new_loc, frozen = self.move_sand()
        if frozen:
            self.frozen_sand_set.add(new_loc)
            self.all_frozen_set.add(new_loc)
            self.mobile_sand = self.sand_origin
            if self.part2 and new_loc == self.sand_origin:
                self.complete()
        elif not self.part2 and self.going_into_abyss(new_loc):
            self.complete()
        else:
            self.mobile_sand = new_loc

        if i is not None:
            if len(self.frozen_sand_set) > 0:
                self.frozen_sand_artist.set_offsets(list(self.frozen_sand_set))
                self.frozen_sand_artist.set_color('red')
            self.mobile_sand_artist.set_offsets([self.mobile_sand])
            self.mobile_sand_artist.set_color('black')
            return self.mobile_sand_artist, self.frozen_sand_artist


def part1(lines, args) -> int:
    sim = Simulation.from_lines(lines)
    if args.plot:
        sim.ani = FuncAnimation(sim.fig, sim.update, frames=None, interval=0,
                                init_func=sim.init_plot, blit=True)
        plt.show()
    else:
        while not sim.completed:
            sim.update(None)
    return len(sim.frozen_sand_set)


def part2(lines, args):
    sim = Simulation.from_lines(lines, part2=True)
    if args.plot:
        sim.ani = FuncAnimation(sim.fig, sim.update, frames=None, interval=0,
                                init_func=sim.init_plot, blit=True)
        plt.show()
    else:
        while not sim.completed:
            sim.update(None)
    return len(sim.frozen_sand_set)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test-input', type=int, default=0)
    parser.add_argument(
        "--log-level",
        default=logging.INFO,
        type=lambda x: getattr(logging, x),
        help="Configure the logging level.",
    )
    parser.add_argument('--plot', action='store_true')
    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)
    if args.test_input > 0:
        with open('test-input'+str(args.test_input), 'r') as infile:
            lines = [line.rstrip() for line in infile.readlines()]
    else:
        from aocd import lines

    a = None
    #a = part1(lines, args)
    b = part2(lines, args)

    print(a)
    print(b)
    if not args.test_input:
        from aocd import submit
        submit(a, part='a')
        submit(b, part='b')

if __name__ == "__main__":
    main()
