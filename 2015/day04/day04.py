import argparse
import hashlib

def get_advent_coin(secret_key, num_zeroes):
    hex_prefix = "0" * num_zeroes
    curr_int = 0
    curr_hash_input = "{}{}".format(secret_key, curr_int)
    while hashlib.md5(curr_hash_input).hexdigest()[0:num_zeroes] != hex_prefix:
        curr_int += 1
        curr_hash_input = "{}{}".format(secret_key, curr_int)
    return curr_int

def get_secret_key(filename):
    with open(filename, 'r') as infile:
        return infile.readline()

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", help="Input file downloaded from Advent of Code website")
    args = parser.parse_args()

    secret_key = get_secret_key(args.input_file)

    print "Part1:", get_advent_coin(secret_key, 5)
    print "Part2:", get_advent_coin(secret_key, 6)

if __name__ == "__main__":
    main()
