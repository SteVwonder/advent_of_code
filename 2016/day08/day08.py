import re
import argparse

import numpy as np

class Display(object):
    def __init__(self, width, height):
        self.array = np.zeros((height, width), dtype=bool)

    def rect(self, width, height):
        print "Rect - w:{}, h:{}".format(width, height)
        self.array[0:height,0:width] = True

    def rotate_row(self, row, amount):
        print "Rotate row - row:{}, amount:{}".format(row, amount)
        self.array[row] = np.roll(self.array[row], amount)

    def rotate_col(self, col, amount):
        print "Rotate col - col:{}, amount:{}".format(col, amount)
        self.array[:,col] = np.roll(self.array[:,col], amount)

    def render(self):
        for row in self.array:
            print "".join(["#" if value else " " for value in row])

def main():
    rect_re = re.compile(r'^rect ([0-9]+)x([0-9]+)$')
    rotate_re = re.compile(r'^rotate (row|column) [xy]=([0-9]+) by ([0-9]+)$')
    display = Display(args.width, args.height)
    with open(args.input_file, 'r') as infile:
        for instruction in infile:
            rect_match = rect_re.match(instruction)
            rotate_match = rotate_re.match(instruction)
            if rect_match:
                display.rect(int(rect_match.group(1)), int(rect_match.group(2)))
            elif rotate_match:
                if rotate_match.group(1) == 'row':
                    display.rotate_row(int(rotate_match.group(2)), int(rotate_match.group(3)))
                elif rotate_match.group(1) == 'column':
                    display.rotate_col(int(rotate_match.group(2)), int(rotate_match.group(3)))
                else:
                    raise RuntimeError("unknown rotate axis", rotate_match.group(1))
            else:
                raise RuntimeError("input doesn't match any known instruction", instruction)
    print "Part 1: {}".format(display.array.sum())
    print "Part 2:"
    display.render()

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    parser.add_argument('--width', type=int, default=50)
    parser.add_argument('--height', type=int, default=6)
    args = parser.parse_args()
    main()

