import re
import argparse

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def get_matrix(input_file):
    lines = get_lines(input_file)

    matrix = []
    for line in lines:
        row = [int(x) for x in re.split(' +', line)]
        matrix.append(row)

    return matrix

def row_is_safe(row):
    diffs = [x-y for (x, y) in zip(row, row[1:])]

    all_increasing = all([x > 0 for x in diffs])
    all_decreasing = all([x < 0 for x in diffs])
    within_threshold = all([abs(x) > 0 and abs(x) <= 3 for x in diffs])

    return (all_increasing or all_decreasing) and within_threshold

def row_is_safe_without_index(row, idx):
    return row_is_safe(row[0:idx] + row[idx+1:])

def indices_with_false(row):
    return [i for i, x in enumerate(row) if not x]

def row_is_safe_with_dampening(row):
    diffs = [x-y for (x, y) in zip(row, row[1:])]

    all_increasing = [x > 0 for x in diffs]
    indices = indices_with_false(all_increasing)
    if len(indices) == 1:
        return row_is_safe_without_index(row, indices[0]) or row_is_safe_without_index(row, indices[0]+1)

    all_decreasing = [x < 0 for x in diffs]
    indices = indices_with_false(all_decreasing)
    if len(indices) == 1:
        return row_is_safe_without_index(row, indices[0]) or row_is_safe_without_index(row, indices[0]+1)

    within_threshold = [abs(x) > 0 and abs(x) <= 3 for x in diffs]
    indices = indices_with_false(within_threshold)
    if len(indices) == 1:
        return row_is_safe_without_index(row, indices[0]) or row_is_safe_without_index(row, indices[0]+1)

    return False

def part1(input_file):
    matrix = get_matrix(input_file)
    safe_rows = sum([row_is_safe(row) for row in matrix])
    return safe_rows

def part2(input_file):
    matrix = get_matrix(input_file)
    safe_rows = sum([row_is_safe(row) or row_is_safe_with_dampening(row) for row in matrix])
    return safe_rows

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
