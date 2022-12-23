import argparse
import logging
from collections.abc import Mapping
from dataclasses import dataclass, field
import math

import numpy as np

@dataclass
class State:
    cycle: int = 1
    X: int = 1
    signal_sum: int = 0
    pending_instructions: Mapping[int, int] = field(default_factory=dict)
    screen: list[str] = field(default_factory=list)

    def incr_cycle(self):
        assert (len(self.screen) + 1) == self.cycle
        logging.debug(f"During cycle {self.cycle: 2d}: "
                      f"CRT draws pixel in position {len(self.screen)}")

        if abs(self.X - ((len(self.screen)) % 40)) <= 1:
            self.screen.append("#")
        else:
            self.screen.append(".")

        current_row_idx = max(int(math.floor((len(self.screen) - 1) / 40)), 0)
        current_row = "".join(self.screen[current_row_idx*40:])
        logging.debug("Current CRT row: "
                      f"{current_row}")

        if ((self.cycle - 20) % 40) == 0:
            self.signal_sum += self.cycle * self.X

        if self.cycle in self.pending_instructions:
            incr = self.pending_instructions[self.cycle]
            self.X += incr
            del self.pending_instructions[self.cycle]
            logging.debug(f"End of cycle {self.cycle: 2d}: "
                          f"finish executing addx {incr}, X is now {self.X}")
            logging.debug("Sprite position: " + "."*(self.X - 1) + "#" * 3 + "."*(40 - self.X - 2))

        self.cycle += 1

    def addx(self, value):
        self.pending_instructions[self.cycle+1] = value
        logging.debug(f"Start cycle  {self.cycle: 2d}: "
                      f"begin executing addx {value}")


def part1(lines):
    state = State()
    for line in lines:
        splits = line.split(" ")
        if splits[0] == "noop":
            state.incr_cycle()
            continue
        elif splits[0] == "addx":
            state.addx(int(splits[1]))
            state.incr_cycle()
            state.incr_cycle()
        else:
            raise RuntimeError(f"Shouldn't happen: {splits}")
    return state.signal_sum


def part2(lines, moviewriter=None):
    state = State()
    for line in lines:
        splits = line.split(" ")
        if splits[0] == "noop":
            state.incr_cycle()
            continue
        elif splits[0] == "addx":
            state.addx(int(splits[1]))
            state.incr_cycle()
            state.incr_cycle()
        else:
            raise RuntimeError(f"Shouldn't happen: {splits}")
    screen_print = ""
    for idx in range(0, len(state.screen), 40):
        screen_print += "".join(state.screen[idx:idx+40]) + "\n"
    return screen_print


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test', type=int, default=0)
    parser.add_argument(
        "--log-level",
        default=logging.INFO,
        type=lambda x: getattr(logging, x),
        help="Configure the logging level.",
    )
    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)
    if args.test > 0:
        with open('test-input'+str(args.test), 'r') as infile:
            lines = [line.strip() for line in infile.readlines()]
    else:
        from aocd import lines

    a = part1(lines)
    b = part2(lines)

    print(a)
    print(b)
    if not args.test:
        from aocd import submit
        submit(a, part='a')

if __name__ == "__main__":
    main()
