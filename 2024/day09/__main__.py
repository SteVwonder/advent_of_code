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

def get_idx_and_size(vals):
    curr_idx = 0
    vals = (int(x) for x in vals)
    for size in vals:
        yield (curr_idx, size)
        curr_idx += size

def get_idx_free_chunk_large_enough(free_chunks, size) -> tuple[int, int, int]:
    for free_idx, (fs_idx, val) in enumerate(free_chunks):
        if val >= size:
            return (free_idx, fs_idx, val)
    return (-1, -1, 0)

def gen_filesystem_layout_p2(input_file):
    line = next(get_lines(input_file)).strip()
    idxs_and_sizes = list(get_idx_and_size(line.strip()))
    allocated = deque(enumerate(itertools.islice(idxs_and_sizes, 0, None, 2)))
    free_chunks = [x for x in itertools.islice(idxs_and_sizes, 1, None, 2) if x[1] > 0]
    filesystem = [-1] * (sum([size for (_,(_,size)) in allocated]) + sum([size for (_,size) in free_chunks]))
    fileID = 0

    for fileID, (fs_idx, size) in allocated:
        for _ in range(size):
            filesystem[fs_idx] = fileID
            fs_idx += 1

    while True:
        try:
            # Moving from the backside
            fileID, (allocated_fs_idx, size) = allocated.pop()
            free_chunks = [(fs_idx, size) for (fs_idx, size) in free_chunks if fs_idx < allocated_fs_idx]
            free_chunks_idx, fs_idx, free_size = get_idx_free_chunk_large_enough(free_chunks, size)
            if fs_idx == -1:
                continue
            else:
                for _ in range(size):
                    filesystem[fs_idx] = fileID
                    fs_idx += 1
                    filesystem[allocated_fs_idx] = -1
                    allocated_fs_idx += 1
                new_size = free_size - size
                if new_size > 0:
                    free_chunks[free_chunks_idx] = (fs_idx, new_size)
                else:
                    free_chunks.pop(free_chunks_idx)

        except IndexError:
            break
    return filesystem

def part2(input_file):
    filesystem_layout = gen_filesystem_layout_p2(input_file)
    return sum((x*y) for (x,y) in enumerate(filesystem_layout) if y >= 0)

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
