import re
import argparse
import itertools

import sys
from os import path
sys.path.append(path.dirname(path.dirname(path.abspath(__file__))))
from modules.parser import default_parser

marker_re = re.compile(r'^\(([0-9]+)x([0-9]+)\)')
def len_decode(line, v2=False):
    total_len, idx = 0, 0
    while (idx < len(line)):
        assert line[idx] != ')'
        if line[idx] == '(':
            match = marker_re.match(line[idx:])
            num_chars = int(match.group(1))
            num_repeates = int(match.group(2))
            idx += len(match.group(0))
            if v2:
                len_chars_to_repeat = len_decode(line[idx:idx+num_chars], True)
            else:
                len_chars_to_repeat = len(line[idx:idx+num_chars])
            total_len += num_repeates * len_chars_to_repeat
            idx += num_chars
        else:
            total_len += 1
            idx += 1
    return total_len

def main():
    with open(args.input_file, 'r') as infile:
        for line in infile:
            print len_decode(line.rstrip())
            print len_decode(line.rstrip(), True)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(parents=[default_parser()])
    args = parser.parse_args()
    main()
