import re
import argparse
from collections import Counter

def checksum_cmp(item1, item2):
    char1, count1 = item1
    char2, count2 = item2
    count_cmp = cmp(count1, count2)
    if count_cmp == 0:
        return cmp(char1, char2) # if same count, then alphabetical
    else:
        return -1 * count_cmp # if counts diff, highest counts first

def name_is_valid(encrypted_name, checksum):
    #counts = Counter([char for char in encrypted_name if (not char == '-')])
    counts = Counter(encrypted_name)
    del counts['-']
    assert len(checksum) == 5
    most_freq_chars = [char for char, count in sorted(counts.iteritems(), cmp=checksum_cmp)[0:5]]
    return checksum == "".join(most_freq_chars)

def shift_word(word, sector_id):
    return "".join([chr(((ord(x) - first_char_num + int(sector_id)) % 26) + first_char_num) for x in word])

first_char_num = ord('a')
def decrypt_name(encrypted_name, sector_id):
    decrypted_name = " ".join(shift_word(word, sector_id) for word in encrypted_name.split('-'))
    return (decrypted_name, sector_id)

def main():
    encryption_re = re.compile(r'([a-z-]+)-([0-9]+)\[([a-z]+)\]')
    with open(args.input_file, 'r') as fp:
        encrypted_matches = [encryption_re.match(line) for line in fp]
    encrypted_matches = [match.groups() for match in encrypted_matches if match is not None]
    filtered_matches = [(encrypted_name, sector_id, checksum) for encrypted_name, sector_id, checksum in encrypted_matches if name_is_valid(encrypted_name, checksum)]
    sector_sum = sum([int(sector_id) for encrypted_name, sector_id, checksum in filtered_matches])

    shifted_matches = [decrypt_name(encrypted_name, sector_id) for encrypted_name, sector_id, checksum in encrypted_matches]

    print "Part 1: {}".format(sector_sum)
    matches_dict = {decrypted_name : sector_id for decrypted_name, sector_id in shifted_matches}
    print "Part 2: {}".format(matches_dict['northpole object storage'])

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    args = parser.parse_args()
    main()
