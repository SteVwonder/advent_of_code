import re
import argparse
import itertools
from tqdm import trange

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def eval_ops(components, operators) -> int:
    ret = components[0]
    for x, op in zip(components[1:], operators):
        ret = op(ret, x)
    return ret

def attempt_solve(test_value, components) -> bool:
    for operators in itertools.product([lambda x,y: x+y, lambda x,y: x*y], repeat=(len(components)-1)):
        if test_value == eval_ops(components, operators):
            return True
    return False

def part1(input_file):
    lines = get_lines(input_file)
    answer = 0
    for line in lines:
        test_value, component_strs = line.split(": ")
        test_value = int(test_value)
        components = [int(x) for x in component_strs.split(" ")]
        if attempt_solve(test_value, components):
            answer += test_value
    return answer

def part2(input_file):
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
