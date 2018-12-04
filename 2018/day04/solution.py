#!/usr/bin/env python3

import re
import sys
import functools
from datetime import datetime
from collections import namedtuple, defaultdict

import numpy as np

def star(f):
    @functools.wraps(f)
    def f_inner(args):
        return f(*args)
    return f_inner

parsing_re = re.compile(r'\[([0-9-]+ [0-9:]+)] (.*)')
def parse_time_from_line(line):
    match_obj = parsing_re.match(line)
    timestamp, event_str = match_obj.group(1, 2)
    time = datetime.strptime(timestamp, '%Y-%m-%d %H:%M')
    return (time, event_str)

event_order = {'Guard': 0,
               'falls' : 1,
               'wakes' : 2}
def parsed_line_sort_key(parsed_line):
    time, event_str = parsed_line
    event_first_word = event_str.split(" ")[0]
    return time, event_order[event_first_word]

guard_re = re.compile(r'Guard #(\d+) begins shift')
def strided_access(lines):
    lines = iter(lines)
    guard_line = next(lines)
    while True:
        try:
            next_line = next(lines)
        except StopIteration:
            return
        # Check if the same guard fell asleep again
        if guard_re.match(next_line[1]) is None:
            # the same guard fell asleep again....slacker!
            yield(guard_line, next_line, next(lines))
        else:
            # we are dealing with a new guard
            # loop in case they didn't fall asleep
            guard_line = next_line

Event = namedtuple("Event", ['id', 'asleep_at', 'awake_at', 'asleep_for'])
def main(filename):
    with open(filename, 'r') as infile:
        lines = [line.strip() for line in infile.readlines()]
    parsed_lines = sorted([parse_time_from_line(line) for line in lines], key=parsed_line_sort_key)

    event_tuples = []
    guards = defaultdict(list)
    for guard_line, asleep_line, wake_line in strided_access(parsed_lines):
        match_obj = guard_re.match(guard_line[1])
        assert match_obj is not None
        curr_guard = int(match_obj.group(1))

        asleep_at = asleep_line[0]
        assert asleep_line[1] == "falls asleep"

        awake_at = wake_line[0]
        assert wake_line[1] == "wakes up"

        asleep_for = ((awake_at - asleep_at).total_seconds() / 60)

        event = Event(curr_guard, asleep_at, awake_at, asleep_for)
        event_tuples.append(event)
        guards[curr_guard].append(event)

    # build a histogram of sleepy minutes for each guard
    sleeping_stats = {}
    for guard_id, event_list in guards.items():
        minutes_asleep_hist = np.zeros(60)
        for event in event_list:
            minutes_asleep_hist[event.asleep_at.minute:event.awake_at.minute] += 1
        sleeping_stats[guard_id] = minutes_asleep_hist

    # sum gets us the guard that slept the most overall
    # max gets us the guard with the sleepiest specific minute
    for idx, func in enumerate([sum, max]):
        sleepiest_guard, sleepiest_hist = max(sleeping_stats.items(), key=star(lambda guard, hist: func(hist)))
        sleepiest_minute = sleepiest_hist.argmax()
        print(f'Part {idx}: {sleepiest_minute * sleepiest_guard}')

if len(sys.argv) > 1:
    main(sys.argv[1])
else:
    main('./input.txt')
