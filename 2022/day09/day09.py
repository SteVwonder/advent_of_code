import argparse
from collections import namedtuple
import logging

import numpy as np

RopeEnd = namedtuple('RopeEnd', ['x', 'y'])
Instruction = namedtuple('Instruction', ['direction', 'distance'])

def move_head(head: RopeEnd, instruction: Instruction) -> RopeEnd:
    if instruction.direction == 'U':
        return head._replace(y=(head.y+instruction.distance))
    elif instruction.direction == 'D':
        return head._replace(y=(head.y-instruction.distance))
    elif instruction.direction == 'R':
        return head._replace(x=(head.x+instruction.distance))
    elif instruction.direction == 'L':
        return head._replace(x=(head.x-instruction.distance))
    else:
        raise RuntimeError(f"Unknown Direction: {instruction.direction}")

def move_tail_linear(head: RopeEnd, tail: RopeEnd, rope_field: str) -> RopeEnd:
    head_pos = getattr(head, rope_field)
    tail_pos = getattr(tail, rope_field)

    if abs(head_pos - tail_pos) <= 1:
        return tail
    elif tail_pos > head_pos:
        tail_pos -= 1
    elif head_pos > tail_pos:
        tail_pos += 1
    else:
        raise RuntimeError(f"Should never happen: {head} {tail}")

    return tail._replace(**{rope_field: tail_pos})

def move_tail_diag(head: RopeEnd, tail: RopeEnd) -> RopeEnd:
    if abs(head.y - tail.y) + abs(head.x - tail.x) <= 2:
        return tail

    if head.y > tail.y:
        new_y = head.y - 1
    else:
        new_y = head.y + 1
    if head.x > tail.x:
        new_x = head.x - 1
    else:
        new_x = head.x + 1

    if (abs(head.y - tail.y) == 2) and (abs(head.x - tail.x) == 2):
        pass
    elif abs(head.y - tail.y) == 2:
        new_x = head.x
    elif abs(head.x - tail.x) == 2:
        new_y = head.y
    else:
        raise RuntimeError(f"Shouldn't happen: {head} {tail}")

    return RopeEnd(x=new_x, y=new_y)

def move_tail(head: RopeEnd, tail: RopeEnd) -> RopeEnd:
    if head.x == tail.x and head.y == tail.y:
        return tail
    elif head.x == tail.x:
        return move_tail_linear(head, tail, 'y')
    elif head.y == tail.y:
        return move_tail_linear(head, tail, 'x')
    else:
        return move_tail_diag(head, tail)

def part1(lines):
    logging.debug("======PART 1======")
    head = tail = RopeEnd(0, 0)
    tail_positions = {tail}

    for line in lines:
        if line.startswith("#"):
            continue
        splits = line.split(" ")
        logging.debug(f"Line: {line}, splits: {splits}")
        instruction = Instruction(splits[0], 1)
        logging.debug(f"\tHead: {head}, Tail: {tail}")
        for _ in range(int(splits[1])):
            logging.debug(f"\tInstruction: {instruction}")
            head = move_head(head, instruction)
            tail = move_tail(head, tail)
            tail_positions.add(tail)
            logging.debug(f"\tHead: {head}, Tail: {tail}")

    return len(tail_positions)

def setup_ax():
    major_ticks = np.arange(-10, 20, 5)
    minor_ticks = np.arange(-10, 20, 1)
    ax = fig.add_subplot(1, 1, 1)
    ax.set_xlim(-10, 20)
    ax.set_ylim(-10, 20)
    ax.set_xticks(major_ticks)
    ax.set_xticks(minor_ticks, minor=True)
    ax.set_yticks(major_ticks)
    ax.set_yticks(minor_ticks, minor=True)
    ax.grid(visible=True, axis='both', which='both')
    return ax

def part2(lines, moviewriter=None):
    logging.debug("======PART 2======")
    snake = [RopeEnd(0, 0) for _ in range(10)]
    tail_positions = {snake[-1]}

    if moviewriter:
        ax = setup_ax()
        time_text = ax.text(20, 20, '', fontsize=15)
        l, = ax.plot([s.x for s in snake], [s.y for s in snake], 'k-o')
    for line_idx, line in enumerate(lines):
        if line.startswith("#"):
            continue
        splits = line.split(" ")
        instruction = Instruction(splits[0], 1)
        logging.debug(f"Line: {line}, splits: {splits}")
        logging.debug(f"\tInstruction: {Instruction(*splits)}")
        for dist_idx in range(int(splits[1])):
            snake[0] = move_head(snake[0], instruction)
            if moviewriter:
                time_text.set_text(f"{line_idx}:{dist_idx}:0")
                l.set_data([s.x for s in snake], [s.y for s in snake])
                moviewriter.grab_frame()
            for idx in range(1, len(snake)):
               new_tail = move_tail(snake[idx-1], snake[idx])
               if snake[idx] != new_tail:
                   snake[idx] = new_tail
                   if moviewriter:
                       time_text.set_text(f"{line_idx}:{dist_idx}:{idx}")
                       l.set_data([s.x for s in snake], [s.y for s in snake])
                       moviewriter.grab_frame()
            tail_positions.add(snake[-1])

    return len(tail_positions)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--test', type=int, default=0)
    parser.add_argument('--plot', action='store_true')
    parser.add_argument(
        "--log-level",
        default=logging.INFO,
        type=lambda x: getattr(logging, x),
        help="Configure the logging level.",
    )
    args = parser.parse_args()

    logging.basicConfig(level=args.log_level)
    if args.test > 0:
        with open('test-input'+str(args.test), 'r') as infile:
            lines = [line.strip() for line in infile.readlines()]
    else:
        from aocd import lines

    a = part1(lines)
    if args.plot:
        from matplotlib import pyplot as plt
        from matplotlib.animation import FFMpegWriter
        global fig
        fig = plt.figure()
        moviewriter = FFMpegWriter(fps=5)
        with moviewriter.saving(fig, 'snake.mp4', dpi=200):
            b = part2(lines, moviewriter)
    else:
        b = part2(lines)

    print(a)
    print(b)
    if not args.test:
        from aocd import submit
        submit(a, part='a')
        submit(b, part='b')

if __name__ == "__main__":
    main()
