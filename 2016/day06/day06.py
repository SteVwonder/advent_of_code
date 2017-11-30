import argparse
from itertools import groupby
from collections import Counter

def unwrap_original_character(item):
    # most_common returns a list with elements of the form: (original_value, count)
    # The first bracket selects the original value
    # The second bracket selects the character (2nd element from enumerate)
    return item[0][1]

def main():
    with open(args.input_file, 'r') as infile:
        key_values = sorted([x for line in infile for x in enumerate(line.rstrip())])
    message1, message2 = list(), list()
    for idx, group in groupby(key_values, key=lambda x: x[0]):
        counter = Counter(group)
        message1.append(unwrap_original_character(counter.most_common(1)[0]))
        message2.append(unwrap_original_character(counter.most_common()[-1]))
    print message1, message2
    print "Part 1: {}".format("".join(message1))
    print "Part 2: {}".format("".join(message2))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()
    main()

