import re
import argparse
from itertools import islice

def window(seq, n=2):
    "Returns a sliding window (of width n) over data from the iterable"
    "   s -> (s0,s1,...s[n-1]), (s1,s2,...,sn), ...                   "
    it = iter(seq)
    result = tuple(islice(it, n))
    if len(result) == n:
        yield result
    for elem in it:
        result = result[1:] + (elem,)
        yield result

def contains_abba(list_of_strings):
    for string in list_of_strings:
        for sequence in window(string, 4):
            if sequence[0] == sequence[3] and sequence[1] == sequence[2] and sequence[0] != sequence[1]:
                return True
    return False

hypernet_re = re.compile(r'\[[a-z]+\]')
bracket_re = re.compile(r'[[\]]')
def get_super_and_hypernets(lines):
    supernets, hypernets = list(), list()
    for line in lines:
        chunks = bracket_re.split(line)
        supernet = [x[1] for x in enumerate(chunks) if x[0] % 2 == 0]
        hypernet = [x[1] for x in enumerate(chunks) if x[0] % 2 == 1]
        #non_hypernet = hypernet_re.split(line)
        #hypernet = hypernet_re.findall(line)
        supernets.append(supernet)
        hypernets.append(hypernet)
    return supernets, hypernets

def supports_tls(supernet, hypernet):
    return contains_abba(supernet) and not contains_abba(hypernet)

def find_abas(list_of_strings):
    abas = []
    for string in list_of_strings:
        for sequence in window(string, 3):
            if sequence[0] == sequence[2] and sequence[0] != sequence[1]:
                abas.append(sequence)
    return abas

def contains_bab(list_of_strings, aba):
    for string in list_of_strings:
        for sequence in window(string, 3):
            if sequence[0] == sequence[2] and sequence[0] != sequence[1] and sequence[0] == aba[1] and sequence[1] == aba[0]:
                return True
    return False

def supports_ssl(supernet, hypernet):
    abas = find_abas(supernet)
    for aba in abas:
        if contains_bab(hypernet, aba):
            return True
    return False

def main():
    with open(args.input_file, 'r') as infile:
        supernets, hypernets  = get_super_and_hypernets(infile)
        num_support_tls = sum([supports_tls(supernet, hypernet) for supernet, hypernet in zip(supernets, hypernets)])
        num_support_ssl = sum([supports_ssl(supernet, hypernet) for supernet, hypernet in zip(supernets, hypernets)])

    print "Part 1: {}".format(num_support_tls)
    print "Part 2: {}".format(num_support_ssl)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()
    main()
