import re
import argparse

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def sum_muls_in_line(line):
    pattern = r'mul\(([0-9]+),([0-9]+)\)'
    sum = 0
    for match in re.finditer(pattern, line):
        sum += int(match.group(1)) * int(match.group(2))
    return sum

def part1(input_file):
    lines = get_lines(input_file)
    return sum([sum_muls_in_line(line) for line in lines])

def sum_muls_in_line_with_conditionals(line, enabled):
    pattern = r"(mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\))"
    sum = 0
    for match in re.finditer(pattern, line):
        if match.group(1) == 'do()':
            enabled = True
        elif match.group(1) == "don't()":
            enabled = False
        elif match.group(1).startswith("mul"):
            if enabled:
                product = int(match.group(2)) * int(match.group(3))
                sum += product
        else:
            raise RuntimeError()
    return sum, enabled

def part2(input_file):
    lines = get_lines(input_file)
    enabled = True
    sum = 0
    for line in lines:
        local_sum, enabled = sum_muls_in_line_with_conditionals(line, enabled)
        sum += local_sum
    return sum

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
