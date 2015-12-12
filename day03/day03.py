import argparse

UP="^"
DOWN="v"
LEFT="<"
RIGHT=">"

def get_unique_houses(directions):
    curr_loc = (0,0)
    locations = set([curr_loc])

    for direction in directions:
        if direction == UP:
            curr_loc = (curr_loc[0], curr_loc[1] + 1)
        elif direction == DOWN:
            curr_loc = (curr_loc[0], curr_loc[1] - 1)
        elif direction == LEFT:
            curr_loc = (curr_loc[0] - 1, curr_loc[1])
        elif direction == RIGHT:
            curr_loc = (curr_loc[0] + 1, curr_loc[1])
        else:
            raise Exception("Unrecognized direction: {}".format(direction))
        locations.add(curr_loc)

    return locations

def get_directions(filename):
    with open(filename, 'r') as infile:
        for line in infile:
            for char in line:
                yield char

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", help="Input file downloaded from Advent of Code website")
    args = parser.parse_args()

    all_directions = list(get_directions(args.input_file))
    santa_dirs = [x for idx, x in enumerate(all_directions) if idx % 2 == 0]
    robot_santa_dirs = [x for idx, x in enumerate(all_directions) if idx % 2 == 1]

    print "Part1:", len(get_unique_houses(all_directions))
    print "Part2:", len( get_unique_houses(santa_dirs).union(get_unique_houses(robot_santa_dirs)) )

if __name__ == "__main__":
    main()
