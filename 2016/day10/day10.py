import re
import argparse
from collections import defaultdict

import sys
from os import path
sys.path.append(path.dirname(path.dirname(path.abspath(__file__))))
from modules.parser import default_parser

class Output(object):
    def __init__(self, identifier):
        self.identifier = identifier
        #self.container = []
        pass

    def __str__(self):
        return "Output {}".format(self.identifier)

    def insert(self, item, process=False):
        #self.container.append(item)
        pass

    def add_parent(self, parent):
        pass

class Bot(object):
    def __init__(self, identifier):
        self.identifier = identifier
        self.container = []
        self.parents = []
        self.high = None
        self.low = None

    def __str__(self):
        return "Bot {}".format(self.identifier)

    def insert(self, item, p3nis=False):
        print "Bot {} is receiving {}".format(self.identifier, item)
        if p3nis and len(self.container) == 2:
            self.process_items(p3nis)
        self.container.append(item)
        if p3nis and len(self.container) == 2:
            self.process_items(p3nis)

    def set_low_high(self, low_obj, high_obj):
        self.low = low_obj
        self.low.add_parent(self)
        self.high = high_obj
        self.high.add_parent(self)

    def add_parent(self, parent):
        self.parents.append(parent)

    def process_items(self, recurse):
        assert len(self.container) == 2
        assert self.low != None and self.high != None, "id: {}, low: {}, high: {}".format(self.identifier, self.low, self.high)
        self.container.sort()
        for goal in GOAL:
            if goal in self.container:
                print "Found {} in bot {}".format(goal, self.identifier)
        if sum([abs(x - y) for x, y in zip(self.container, GOAL)]) == 0:
            print "Part 1: Bot #{}, container {}, goal {}".format(self.identifier, self.container, GOAL)
        print "Sending {} to {} and {} to {}".format(self.container[0], self.low, self.container[1], self.high)
        self.low.insert(self.container.pop(), recurse)
        self.high.insert(self.container.pop(), recurse)

class keydefaultdict(defaultdict):
    def __missing__(self, key):
        if self.default_factory is None:
            raise KeyError( key )
        else:
            ret = self[key] = self.default_factory(key)
            return ret

def main():
    global GOAL
    GOAL = args.goal
    bot_dict = keydefaultdict(Bot)
    bot_re = re.compile(r'^bot ([0-9]+) gives low to (output|bot) ([0-9]+) and high to (output|bot) ([0-9]+)$')
    value_re = re.compile(r'^value ([0-9]+) goes to bot ([0-9]+)$')
    with open(args.input_file, 'r') as infile:
        for instruction in (line.rstrip() for line in infile):
            if instruction[0] == 'b':
                match = bot_re.match(instruction)
                bot_id, low_type, low_id, high_type, high_id = match.groups()
                low_obj = bot_dict[low_id] if low_type == 'bot' else Output(low_id)
                high_obj = bot_dict[high_id] if high_type == 'bot' else Output(high_id)
                print "Set bot {}'s low to {} and high to {}".format(bot_id, low_id, high_id)
                bot_dict[bot_id].set_low_high(low_obj, high_obj)
            else:
                match = value_re.match(instruction)
                value_id, bot_id = match.groups()
                print "Inserting {} into {}".format(value_id, bot_id)
                bot_dict[bot_id].insert(int(value_id), False)
    roots = [bot for bot in bot_dict.itervalues() if len(bot.parents) == 0]
    for root in roots:
        root.process_items(True)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(parents=[default_parser()])
    parser.add_argument('goal', nargs=2, type=int)
    args = parser.parse_args()
    main()
