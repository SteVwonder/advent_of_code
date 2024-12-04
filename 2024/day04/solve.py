import re
import argparse
import itertools

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def get_matrix(input_file):
    lines = get_lines(input_file)

    matrix = []
    for line in lines:
        row = [x.lower() for x in line.strip()]
        matrix.append(row)

    return Matrix(matrix)

def idx_generator(starting_idx, direction):
    idx = list(starting_idx)
    while True:
        yield idx
        idx[0] += direction[0]
        idx[1] += direction[1]

def take(n, iterable):
    "Return first n items of the iterable as a list."
    return list(itertools.islice(iterable, n))

class Matrix:
    def __init__(self, matrix):
        self.matrix = matrix

    def get(self, x, y):
        if x < 0 or y < 0:
            return '.'
        try:
            return self.matrix[x][y]
        except IndexError:
            return '.'

    def get_word_at(self, starting_idx, direction, length=None):
        indices = idx_generator(starting_idx, direction)
        search_word = (self.get(x, y) for (x, y) in indices)
        if length is not None:
            search_word = take(length, search_word)
        return search_word

def search_for_target(matrix, starting_idx, target=['x', 'm', 'a', 's']):
    search_directions = list(itertools.product([-1, 0, 1], [-1, 0, 1]))
    search_directions.remove((0,0))

    targets_found = 0
    for search_direction in search_directions:
        search_word = matrix.get_word_at(starting_idx, search_direction)
        found = all((x == y for (x, y) in zip(target, search_word)))
        targets_found += found
    return targets_found

def part1(input_file):
    matrix = get_matrix(input_file)
    sum = 0
    for row_idx, row in enumerate(matrix.matrix):
        for col_idx, _ in enumerate(row):
            # it isn't necessary, but definitely speeds things up
            if matrix.matrix[row_idx][col_idx] == 'x':
                sum += search_for_target(matrix, (row_idx, col_idx))
    return sum

def part2(input_file):
    matrix = get_matrix(input_file)
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
