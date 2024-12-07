import re
import argparse
import itertools

def get_lines(input_file):
    with open(input_file, 'r') as infile:
        for line in infile:
            yield line

def get_input(input_file) -> list[tuple[int, list[int]]]:
    input = []
    lines = get_lines(input_file)
    for line in lines:
        test_value, component_strs = line.split(": ")
        test_value = int(test_value)
        components = [int(x) for x in component_strs.split(" ")]
        input.append((test_value, components))
    return input

def eval_ops(components, operators) -> int:
    ret = components[0]
    for x, op in zip(components[1:], operators):
        ret = op(ret, x)
    return ret

def attempt_solve(test_value, components, functions) -> bool:
    for operators in itertools.product(functions, repeat=(len(components)-1)):
        if test_value == eval_ops(components, operators):
            return True
    return False

def part1(input_file):
    answer = 0
    for (test_value, components) in get_input(input_file):
        if attempt_solve(test_value, components, [lambda x,y: x+y, lambda x,y: x*y]):
            answer += test_value
    return answer

def part2(input_file):
    answer = 0
    for (test_value, components) in get_input(input_file):
        if attempt_solve(test_value, components, [lambda x,y: x+y, lambda x,y: x*y, lambda x,y: int(str(x) + str(y))]):
            answer += test_value
    return answer

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
