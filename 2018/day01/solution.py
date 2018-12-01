#!/usr/bin/env python3

def main():
    with open('input.txt', 'r') as infile:
        lines = infile.readlines()
    freq = 0
    for line in lines:
        delta = int(line)
        freq += delta
    print(f'Part 1: {freq}')

    freq = 0
    freq_set = set([0])
    found = False
    while not found:
        for line in lines:
            delta = int(line)
            freq += delta
            if freq in freq_set:
                print(f'Part 2: {freq}')
                found = True
                break
            freq_set.add(freq)

if __name__ == "__main__":
    main()
