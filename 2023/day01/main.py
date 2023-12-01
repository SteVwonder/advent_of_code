import re
import argparse


digits_re = r'[0-9]'
def get_part1_num(line: str) -> int:
    numbers = re.findall(digits_re, line)
    composite = numbers[0] + numbers[-1]
    return int(composite)

words_re = r'one|two|three|four|five|six|seven|eight|nine'
all_re = "{}|{}".format(digits_re, words_re, words_re)
word_to_digit_map = {
    'one': "1",
    'two': "2",
    'three': "3",
    'four': "4",
    'five': "5",
    'six': "6",
    'seven': "7",
    'eight': "8",
    'nine': "9",
}
def get_digit_from_match(match: str) -> str:
    return word_to_digit_map.get(match, match)

def overlapping_matches(line):
    while True:
        next_match = re.search(all_re, line)
        if next_match is None:
            break
        idx = next_match.start() + 1
        line = line[idx:]
        yield next_match[0]

def get_part2_num(line: str) -> int:
    all_matches = list(overlapping_matches(line))
    digits = "".join([
        get_digit_from_match(all_matches[x]) for x in [0, -1]
    ])
    return int(digits)

def main(args):
    accum = 0
    with open(args.input_file) as fp:
        for line in fp:
            if args.part2:
                accum += get_part2_num(line)
            else:
                accum += get_part1_num(line)
    print(accum)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    parser.add_argument('--part2', action='store_true')
    args = parser.parse_args()
    main(args)
