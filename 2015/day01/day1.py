import argparse

def get_chars(filename):
    with open(filename, 'r') as infile:
        for line in infile:
            for char in line:
                yield char

def count_floors(chars, part2=False):
    curr_floor = 0
    for pos, char in enumerate(chars):
        if char == "(":
            curr_floor += 1
        elif char == ")":
            curr_floor -= 1
        else:
            raise Exception("Unrecognized input char")

        if part2 and curr_floor == -1:
            return pos + 1 #enumerate is 0 indexed

    return curr_floor

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", help="Input file downloaded from Advent of Code website")
    parser.add_argument("--part2", help="Solve Part 2 rather than Part 1", action="store_true")
    args = parser.parse_args()
    print count_floors(get_chars(args.input_file), part2=args.part2)

if __name__ == "__main__":
    main()
