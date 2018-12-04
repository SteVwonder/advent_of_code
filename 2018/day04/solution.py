#!/usr/bin/env python3

import re
import sys
from datetime import datetime
from collections import namedtuple, defaultdict

import numpy as np

parsing_re = re.compile(r'\[([0-9-]+ [0-9:]+)] (.*)')
def parse_line(line):
    match_obj = parsing_re.match(line)
    timestamp, event_str = match_obj.group(1, 2)
    time = datetime.strptime(timestamp, '%Y-%m-%d %H:%M')
    return (time, event_str)

event_order = {'G': 0,
               'f' : 1,
               'w' : 2}
def parsed_line_sort_key(parsed_line):
    time, event_str = parsed_line
    return time, event_order[event_str[0]]

def main(filename):
    with open(filename, 'r') as infile:
        lines = [line.strip() for line in infile.readlines()]

    Event = namedtuple("Event", ['id', 'asleep_at', 'awake_at', 'asleep_for'])

    parsed_lines = sorted([parse_line(line) for line in lines], key=parsed_line_sort_key)

    guard_re = re.compile(r'Guard #(\d+) begins shift')
    curr_guard = -1
    asleep_at = -1
    awake_at = -1
    event_tuples = []
    guards = defaultdict(list)
    for time, event_str in parsed_lines:
        match_obj = guard_re.match(event_str)
        if match_obj is not None:
            curr_guard = int(match_obj.group(1))
            asleep_at = -1
            awake_at = -1
        elif event_str == "falls asleep":
            asleep_at = time
        elif event_str == "wakes up":
            awake_at = time
            if asleep_at == -1:
                print(f"Guard {curr_guard} doesn't have an asleep time")
            asleep_for = ((awake_at - asleep_at).total_seconds() / 60)
            event = Event(curr_guard, asleep_at, awake_at, asleep_for)
            event_tuples.append(event)
            guards[curr_guard].append(event)

    total_time_asleep = defaultdict(lambda: 0)
    for guard_id, event_list in guards.items():
        for event in event_list:
            total_time_asleep[guard_id] += event.asleep_for
    sleepiest_guard = max(total_time_asleep.items(), key=lambda x: x[1])[0]

    minutes_asleep_hist = np.zeros(60)
    for event in guards[sleepiest_guard]:
        minutes_asleep_hist[event.asleep_at.minute:event.awake_at.minute] += 1
    sleepiest_minute = minutes_asleep_hist.argmax()
    print(f'Part 1: {sleepiest_minute * sleepiest_guard}')


    strat2_tuples = []
    for guard_id, event_list in guards.items():
        minutes_asleep_hist = np.zeros(60)
        for event in event_list:
            minutes_asleep_hist[event.asleep_at.minute:event.awake_at.minute] += 1
        sleepiest_minute = minutes_asleep_hist.argmax()
        strat2_tuples.append((minutes_asleep_hist[sleepiest_minute], sleepiest_minute, guard_id))
    strat2_matching_tuple = max(strat2_tuples, key=lambda x: x[0])
    print(f'Part 2: {strat2_matching_tuple[1]*strat2_matching_tuple[2]}')

if len(sys.argv) > 1:
    main(sys.argv[1])
else:
    main('./input.txt')
