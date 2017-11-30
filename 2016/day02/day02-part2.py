class KeypadPosition(object):
    keymap = [
        [None, None, 1 , None, None],
        [None,   2 , 3 ,  4  , None],
        [ 5  ,   6 , 7 ,  8  ,  9  ],
        [None,  "A","B", "C" , None],
        [None, None,"D", None, None]]

    def __init__(self, x=1, y=1):
        self.x = x
        self.y = y

    def __iter__(self):
        yield self.x
        yield self.y

    def __str__(self):
        return "<{}, {}>".format(self.x, self.y)

    def validate_position(self):
        try:
            assert (self.x + self.y) <= 6
            assert 4 >= self.x >= 0
            assert 4 >= self.y >= 0
            assert KeypadPosition.keymap[self.y][self.x] is not None
        except AssertionError:
            print self
            raise

    def move_vertically(self, amount=1):
        assert abs(amount) == 1
        new_value = self.y + amount
        if new_value > 4 or new_value < 0:
            pass
        elif KeypadPosition.keymap[new_value][self.x] is not None:
            self.y = new_value
        self.validate_position()

    def move_horizontally(self, amount=1):
        assert abs(amount) == 1
        new_value = self.x + amount
        if new_value > 4 or new_value < 0:
            pass
        elif KeypadPosition.keymap[self.y][new_value] is not None:
            self.x = new_value
        self.validate_position()

    @property
    def tuple(self):
        return (self.x, self.y)

    @property
    def curr_keycode(self):
        return str(KeypadPosition.keymap[self.y][self.x])

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
        return self.position.curr_keycode

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
    print "Part 2: {}".format("".join(keypad_commbination))

if __name__ == "__main__":
    main()
