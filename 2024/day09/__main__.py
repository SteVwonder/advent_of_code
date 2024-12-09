import os
import argparse

import itertools
from collections import deque
from typing import Generator

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

class Allocated:
    def __init__(self, allocated):
        self.allocated = deque(list(enumerate(allocated)))

    def num_in_front(self) -> tuple[int, int]:
        return self.allocated.popleft()

    def next_from_back(self) -> Generator[int, None, None]:
        curr_back = self.allocated.pop()
        while True:
            for _ in range(curr_back[1]):
                yield curr_back[0]
            try:
                curr_back = self.allocated.pop()
            except IndexError:
                break

def gen_filesystem_layout(input_file):
    line = next(get_lines(input_file)).strip()
    allocated = Allocated([int(x) for x in itertools.islice(line, 0, None, 2)])
    free_chunks = deque([int(x) for x in itertools.islice(line, 1, None, 2)])
    fileID = 0
    backside = allocated.next_from_back()
    while True:
        try:
            (fileID, num) = allocated.num_in_front()
            for _ in range(num):
                yield fileID
            curr_free = free_chunks.popleft()
        except IndexError:
            for back in backside:
                yield back
            break
        for _ in range(curr_free):
            fileID = next(backside, None)
            if fileID is None:
                break
            yield fileID

def part1(input_file):
    filesystem_layout = gen_filesystem_layout(input_file)
    return sum((x*y) for (x,y) in enumerate(filesystem_layout))

def part2(input_file):
    pass

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
