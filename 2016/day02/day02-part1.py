class KeypadPosition(object):
    keymap = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]]

    def __init__(self, x=1, y=1):
        self.x = x
        self.y = y

    def __iter__(self):
        yield self.x
        yield self.y

    def __str__(self):
        return "<{}, {}>".format(self.x, self.y)

    def move_vertically(self, amount=1):
        self.y = min(max(0, self.y + amount), 2)

    def move_horizontally(self, amount=1):
        self.x = min(max(0, self.x + amount), 2)

    @property
    def tuple(self):
        return (self.x, self.y)

    @property
    def number(self):
        return KeypadPosition.keymap[self.y][self.x]

class Walker(object):
    def __init__(self):
        self.position = KeypadPosition(1,1)

    def walk(self, instruction):
        if instruction == 'U':
            self.position.move_vertically(-1)
        elif instruction == 'R':
            self.position.move_horizontally(1)
        elif instruction == 'D':
            self.position.move_vertically(1)
        elif instruction == 'L':
            self.position.move_horizontally(-1)

    @property
    def curr_keypad_number(self):
        return self.position.number

def main():
    with open('input.txt', 'r') as fp:
        instruction_streams = [line.rstrip() for line in fp]
    walker = Walker()
    keypad_commbination = []
    for instruction_stream in instruction_streams:
        for instruction in instruction_stream:
            walker.walk(instruction)
            #print "Instruction: {}, NewPosition: {}, Key: {}".format(instruction, walker.position, walker.curr_keypad_number)
        keypad_commbination.append(str(walker.curr_keypad_number))
    print "Part 1: {}".format("".join(keypad_commbination))
    #print "Part 2: {}".format(first_overlap_distance)

if __name__ == "__main__":
    main()
