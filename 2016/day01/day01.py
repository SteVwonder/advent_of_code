class Position(object):
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y

    def __iter__(self):
        yield self.x
        yield self.y

    def __str__(self):
        return "<{}, {}>".format(self.x, self.y)

    @property
    def tuple(self):
        return (self.x, self.y)

    @property
    def blocks_from_origin(self):
        return sum([abs(x) for x in iter(self)])

class Walker(object):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

    def __init__(self):
        self.position = Position(0,0)
        self.direction = Walker.NORTH
        self.previous_locations = set([self.position.tuple])
        #self.previous_locations = [self.position.tuple]

    def turn(self, direction_str):
        if direction_str == 'L':
            self.direction -= 1
        elif direction_str == 'R':
            self.direction += 1
        else:
            raise RuntimeError('Unknown direction: {}'.format(direction_str))
        self.direction = self.direction % 4

    def walk(self, length):
        if self.direction == Walker.NORTH:
            self.position.y += length
        elif self.direction == Walker.EAST:
            self.position.x += length
        elif self.direction == Walker.SOUTH:
            self.position.y -= length
        elif self.direction == Walker.WEST:
            self.position.x -= length
        else:
            raise RuntimeError('Invalid direction: {}'.format(self.direction))

    def check_overlap(self):
        if self.position.tuple in self.previous_locations:
            return self.position.tuple
        return None

    def walk_and_check_overlap(self, length):
        overlap = None
        for x in xrange(length):
            self.walk(1)
            if overlap is None:
                overlap = self.check_overlap()
            self.previous_locations.add(self.position.tuple)
        return overlap

    @property
    def blocks_from_origin(self):
        return self.position.blocks_from_origin

    def plot_path(self):
        import matplotlib.pyplot as plt
        xs, ys = zip(*self.previous_locations)
        plt.plot(xs, ys)
        plt.show()

def main():
    with open('input.txt', 'r') as fp:
        line = fp.readline().rstrip()
    instructions = line.split(', ')
    walker = Walker()
    first_overlap_distance = None
    for instruction in instructions:
        direction = instruction[0]
        length = int(instruction[1:])
        walker.turn(direction)
        overlap = walker.walk_and_check_overlap(length)
        if first_overlap_distance is None and overlap is not None:
            first_overlap_distance = Position(*overlap).blocks_from_origin
    print "Part 1: {}".format(walker.blocks_from_origin)
    print walker.position.tuple
    print "Part 2: {}".format(first_overlap_distance)

if __name__ == "__main__":
    main()
