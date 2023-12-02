import argparse
from collections import defaultdict

type Game = list[dict[str, int]]

target_bag = {
    'red': 12,
    'green': 13,
    'blue': 14
}
def game_is_possible(game: Game) -> bool:
    for round in game:
        for color, count in round.items():
            if count > target_bag[color]:
                return False
    return True

def parse_line(line: str) -> tuple[int, Game]:
    game = []
    prefix, line = line.split(':')
    game_id = int(prefix.split()[1])
    for round_str in line.split(';'):
        round = defaultdict(int)
        for move in [x.strip() for x in round_str.split(',')]:
            count, color = move.strip().split()
            round[color] += int(count)
        game.append(round)
    return game_id, game

def main(args):
    score = 0
    with open(args.input_file) as fp:
        for line in fp:
            game_id, game = parse_line(line)
            possible = game_is_possible(game)
            if args.verbose:
                print(possible, game_id, game)
            if possible:
                score += game_id
    print(score)

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('input_file')
    parser.add_argument('--part2', action='store_true')
    parser.add_argument('-v', '--verbose', action='store_true')
    args = parser.parse_args()
    main(args)
