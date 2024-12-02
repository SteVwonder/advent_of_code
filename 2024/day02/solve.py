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

def get_all_conditions(row):
    diffs = [x-y for (x, y) in zip(row, row[1:])]

    increasing = [x > 0 for x in diffs]
    decreasing = [x < 0 for x in diffs]
    within_threshold = [abs(x) > 0 and abs(x) <= 3 for x in diffs]

    return (increasing, decreasing, within_threshold)

def row_is_safe(row):
    increasing, decreasing, within_threshold =  get_all_conditions(row)
    return (all(increasing) or all(decreasing)) and all(within_threshold)

def row_is_safe_without_index(row, idx):
    return row_is_safe(row[0:idx] + row[idx+1:])

def indices_with_false(row):
    return [i for i, x in enumerate(row) if not x]

def row_is_safe_with_dampening(row):
    conditions = get_all_conditions(row)
    for condition in conditions:
        indices = indices_with_false(condition)
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
