import argparse

def get_chars(filename):
    with open(filename, 'r') as infile:
        for line in infile:
            for char in line:
                yield char

def count_floors(chars):
    curr_floor = 0
    for char in chars:
        if char == "(":
            curr_floor += 1
        elif char == ")":
            curr_floor -= 1
        else:
            raise Exception("Unrecognized input char")
    return curr_floor

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", help="Input file downloaded from Advent of Code website")
    args = parser.parse_args()
    print count_floors(get_chars(args.input_file))

if __name__ == "__main__":
    main()
