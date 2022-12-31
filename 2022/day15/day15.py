#!/bin/env python3

import re
import argparse
import logging
from collections import defaultdict, namedtuple
from collections.abc import Mapping, Callable
from dataclasses import dataclass, field

from tqdm import trange, tqdm
import numpy as np
from matplotlib import pyplot as plt
from matplotlib.ticker import MultipleLocator

Point = namedtuple("Point", ['x', 'y'])
Sensor = namedtuple("Sensor", ['x', 'y', 'dist'])

def manhattan_dist(self: Point|Sensor, other: Point|Sensor) -> int:
    return abs(self.x - other.x) + abs(self.y - other.y)


class Grid:
    def __init__(self, beacons: Mapping[Point, set[Sensor]], sensors: list[Sensor]):
        self.beacons = beacons
        self.sensors = sensors
        smallest = next(iter(sensors))
        largest = smallest
        for sensor in sensors:
            distance = sensor.dist
            smallest = Point(
                min(smallest.x, sensor.x - distance),
                min(smallest.y, sensor.y - distance)
            )
            largest = Point(
                max(largest.x, sensor.x + distance),
                max(largest.y, sensor.y + distance)
            )
        self.xlim = (smallest.x-1, largest.x+1)
        self.ylim = (smallest.y-1, largest.y+1)

    @staticmethod
    def from_lines(lines):
        sensors = set[Sensor]()
        beacons = defaultdict(list)

        line_re = re.compile(r'Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)')
        for line in lines:
            match = line_re.match(line)
            if match is None or None in match.groups():
                raise RuntimeError(f"Failed to parse line: {line}")
            sensor_point = Point(*[int(x) for x in match.group(1, 2)])
            beacon = Point(*[int(x) for x in match.group(3, 4)])
            dist = manhattan_dist(sensor_point, beacon)
            sensor = Sensor(sensor_point.x, sensor_point.y, dist)
            sensors.add(sensor)
            beacons[beacon].append(sensor)
        return Grid(beacons, sensors)

    def plot(self):
        fig, ax = plt.subplots()
        ax.set_xlim(*self.xlim)
        ax.set_ylim(*self.ylim)
        ax.xaxis.set_major_locator(MultipleLocator(5))
        ax.xaxis.set_minor_locator(MultipleLocator(1))
        ax.grid(visible=True, which='both', axis='both', dashes=(4, 2), dash_joinstyle='round')
        ax.invert_yaxis()

        for beacon in self.beacons.keys():
            ax.text(*beacon, "B")

        for sensor in self.sensors:
            ax.text(sensor.x, sensor.y, "S")

        for covered in self.covered_spaces:
            if covered not in self.beacons and not self.sensor_at(covered):
                ax.text(*covered, "#")

        plt.show()

    def sensor_at(self, point: Point) -> bool:
        for sensor in self.sensors:
            if manhattan_dist(sensor, point) == 0:
                return True
        return False

    def is_covered(self, point: Point) -> tuple[bool, Sensor | None]:
        # TODO: convert sensors to an LRU based on matches?
        for sensor in self.sensors:
            curr_pos_dist = manhattan_dist(sensor, point)
            if curr_pos_dist <= sensor.dist:
                return True, sensor
        return False, None

    @property
    def covered_spaces(self):
        covered = set()
        for sensor in self.sensors:
            start_x = end_x = sensor[0]
            distance = sensor.dist
            for dist in range(distance * 2 + 1):
                y = sensor[1] + distance - dist
                for x in range(start_x, end_x + 1):
                    covered.add(Point(x, y))
                if dist >= distance:
                    start_x += 1
                    end_x -= 1
                else:
                    start_x -= 1
                    end_x += 1
        return covered

    def find_num_covered(self, y, x_min, x_max) -> int:
        sum = 0
        target = x_max+1
        x = x_min
        with tqdm(total=(target - x), leave=False) as pbar:
            while x < target:
                point = Point(x, y)
                covered, sensor = self.is_covered(point)
                if covered:
                    assert sensor is not None
                    vertical_dist = abs(y - sensor.y)
                    horizontal_dist = sensor.x - x
                    covered = max(horizontal_dist + (sensor.dist - vertical_dist), 1)
                    sum += covered
                    x += covered
                    pbar.update(covered)
                else:
                    x += 1
                    pbar.update(1)
        return sum

    def find_uncovered_at_y(self, y, x_min, x_max) -> Point | None:
        target = x_max+1
        x = x_min
        with tqdm(total=(target - x), leave=False) as pbar:
            while x < target:
                point = Point(x, y)
                covered, sensor = self.is_covered(point)
                if covered:
                    assert sensor is not None
                    vertical_dist = abs(y - sensor.y)
                    horizontal_dist = sensor.x - x
                    covered = max(horizontal_dist + (sensor.dist - vertical_dist), 1)
                    x += covered
                    pbar.update(covered)
                else:
                    return point
        return None

    def find_uncovered(self, x_max, y_max, x_min=0, y_min=0) -> Point | None:
        for y in trange(y_min, y_max + 1):
            uncovered = self.find_uncovered_at_y(y, x_min, x_max)
            if uncovered is not None:
                return uncovered
        return None

def part1(lines, args) -> int:
    grid = Grid.from_lines(lines)
    if args.plot:
        grid.plot()

    target_y = 2000000
    if args.test_input:
        target_y = 10
    covered = grid.find_num_covered(target_y, grid.xlim[0], grid.xlim[1])
    beacons = sum((1 if beacon[1] == target_y else 0 for beacon in grid.beacons))
    return covered - beacons

def part2(lines, args) -> int:
    grid = Grid.from_lines(lines)

    x_max = y_max = 4000000
    if args.test_input:
        x_max = y_max = 20
    uncovered = grid.find_uncovered(x_max, y_max)
    assert uncovered is not None
    return uncovered.x * 4000000 + uncovered.y


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

    a = part1(lines, args)
    b = part2(lines, args)

    print(a)
    print(b)
    if not args.test_input:
        from aocd import submit
        submit(a, part='a')
        submit(b, part='b')

if __name__ == "__main__":
    main()
