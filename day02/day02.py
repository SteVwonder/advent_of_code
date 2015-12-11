import argparse

class Box(object):
    def __init__(self, l, w, h):
        self.l = l
        self.w = w
        self.h = h

    def needed_paper(self):
        return 2*self.l*self.w + 2*self.w*self.h + 2*self.h*self.l + \
            min(self.l*self.w, self.l*self.h, self.w*self.h)

    def needed_ribbon(self):
        return self.w*self.h*self.l + \
            2 * min(self.l+self.w, self.l+self.h, self.w+self.h)

def calc_needed_paper(boxes):
    return sum([box.needed_paper() for box in boxes])

def calc_needed_ribbon(boxes):
    return sum([box.needed_ribbon() for box in boxes])

def get_boxes(filename):
    with open(filename, 'r') as infile:
        return [Box(*map(int, line.split("x"))) for line in infile]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("input_file", help="Input file downloaded from Advent of Code website")
    args = parser.parse_args()
    boxes = get_boxes(args.input_file)
    print "Part1:", calc_needed_paper(boxes)
    print "Part2:", calc_needed_ribbon(boxes)

if __name__ == "__main__":
    main()
