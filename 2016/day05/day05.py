import argparse
import hashlib

def main():
    hasher = hashlib.md5()
    hasher.update(args.door_id)
    password1 = []
    password2 = [None] * 8
    password2_count = 0
    curr_idx = 0
    while len(password1) < 8 or password2_count < 8:
        curr_hasher = hasher.copy()
        curr_hasher.update(str(curr_idx))
        curr_hash = curr_hasher.hexdigest()
        if curr_hash[0:5] == "00000":
            if len(password1) < 8:
                password1.append(curr_hash[5])
            position = curr_hash[5]
            if position.isdigit():
                position = int(position)
                if position < 8 and password2[position] is None:
                    password2[position] = curr_hash[6]
                    password2_count += 1
        curr_idx += 1
    print "Part 1: {}".format("".join(password1))
    print "Part 2: {}".format("".join(password2))

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('door_id')
    args = parser.parse_args()
    main()
