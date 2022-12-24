import argparse
import logging
import re
from collections.abc import Mapping, Callable
from dataclasses import dataclass, field
import math
from operator import mul, add

@dataclass
class Monkey:
    num: int
    items: list
    operator: Callable[[int], int]
    operand: int
    test_divisor: int
    true_target: int
    false_target: int
    items_inspected: int = 0

    @staticmethod
    def from_regex_matches(match):
        groups = match.groups()
        items = [int(x) for x in groups[1].split(", ")]
        if groups[3] == 'old':
            operator = math.pow
            operand = 2
        elif groups[2] == "*":
            operator = mul
            operand = int(groups[3])
        elif groups[2] == "+":
            operator = add
            operand = int(groups[3])
        else:
            raise RuntimeError("Unknown operator/operand combo")
        return Monkey(
            groups[0], items,
            operator, operand,
            int(groups[4]), int(groups[5]), int(groups[6])
        )

    def inspect_items(self, monkeys):
        while len(self.items) > 0:
            item = self.items.pop(0)
            self.items_inspected += 1
            new_worry = int(self.operator(item, self.operand) // 3)
            if new_worry % self.test_divisor == 0:
                monkeys[self.true_target].items.append(new_worry)
            else:
                monkeys[self.false_target].items.append(new_worry)

monkey_re = re.compile(
    r'Monkey (\d+):\n'
    r'  Starting items: ([0-9, ]+)\n'
    r'  Operation: new = old ([*+]) (\d+|old)\n'
    r'  Test: divisible by (\d+)\n'
    r'    If true: throw to monkey (\d+)\n'
    r'    If false: throw to monkey (\d+)',
    re.MULTILINE
)


def part1(lines):
    monkeys = []
    for match in monkey_re.finditer("\n".join(lines)):
        monkeys.append(Monkey.from_regex_matches(match))
        logging.debug(f'Monkey: {monkeys[-1]}')
    assert len(monkeys) > 0
    for round in range(1, 21):
        for monkey in monkeys:
            monkey.inspect_items(monkeys)
        logging.debug(f'==========ROUND {round}==========')
        for monkey in monkeys:
            logging.debug(f'\t{monkey}')
    items_inspected = sorted([m.items_inspected for m in monkeys], reverse=True)
    return items_inspected[0] * items_inspected[1]


def part2(lines):
    pass


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
            lines = [line.rstrip() for line in infile.readlines()]
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
