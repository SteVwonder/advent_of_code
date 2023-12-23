use std::collections::VecDeque;
use std::env;

fn num_rounded_rocks(line: &Vec<char>) -> u32 {
    line.iter()
        .filter_map(|c: &char| {
            if *c == 'O' {
                return Some(1);
            }
            None
        })
        .sum()
}

fn calc_weight(lines: &Vec<Vec<char>>) -> u32 {
    let num_rows = lines.len() as u32;
    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| num_rounded_rocks(line) * (num_rows - idx as u32))
        .sum()
}

fn move_rocks(lines: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output = Vec::<Vec<char>>::new();
    for idx in 0..lines.len() {
        output.push(Vec::<char>::new());
    }

    for col_idx in 0..lines[0].len() {
        let col = lines
            .iter()
            .map(|line| line[col_idx])
            .rev()
            .collect::<Vec<_>>();
        let mut new_col = VecDeque::new();
        let mut num_blank = 0;
        let mut num_round_rocks = 0;
        for val in col.iter() {
            match val {
                'O' => num_round_rocks += 1,
                '.' => num_blank += 1,
                '#' => {
                    for _ in 0..num_blank {
                        new_col.push_front('.');
                    }
                    for _ in 0..num_round_rocks {
                        new_col.push_front('O');
                    }
                    new_col.push_front('#');
                    num_round_rocks = 0;
                    num_blank = 0;
                }
                _ => unreachable!(),
            }
        }
        for _ in 0..num_blank {
            new_col.push_front('.');
        }
        for _ in 0..num_round_rocks {
            new_col.push_front('O');
        }
        for (val, row) in new_col.iter().zip(output.iter_mut()) {
            row.push(*val);
        }
    }

    output
}

fn part1(lines: &Vec<Vec<char>>) -> u32 {
    let new_lines = move_rocks(lines);
    calc_weight(&new_lines)
}

fn part2(lines: Vec<Vec<char>>) -> u32 {
    0
}

fn read_file_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.chars().collect())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let lines = read_file_lines(filename);
    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    type Example = Vec<Vec<char>>;

    fn get_part1_examples() -> Vec<Example> {
        let examples = vec![
            vec![
                "O....#....",
                "O.OO#....#",
                ".....##...",
                "OO.#O....O",
                ".O.....O#.",
                "O.#..O.#.#",
                "..O..#O..O",
                ".......O..",
                "#....###..",
                "#OO..#....",
            ],
            vec![
                "OOOO.#.O..",
                "OO..#....#",
                "OO..O##..O",
                "O..#.OO...",
                "........#.",
                "..#....#.#",
                "..O..#.O.O",
                "..O.......",
                "#....###..",
                "#....#....",
            ],
        ];
        examples
            .into_iter()
            .map(|example| {
                example
                    .into_iter()
                    .map(|line: &str| (*line).chars().collect::<Vec<char>>())
                    .collect::<Example>()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_calc_weight() {
        let examples = get_part1_examples();
        assert_eq!(calc_weight(&examples[1]), 136,);
    }

    #[test]
    fn test_move_rocks() {
        let examples = get_part1_examples();
        let moved = move_rocks(&examples[0]);
        assert_eq!(moved, examples[1]);
    }

    #[test]
    fn test_part1_e2e() {
        let examples = get_part1_examples();
        let expected: Vec<u32> = vec![136, 136];
        for (example, expected) in examples.iter().zip(expected.iter()) {
            assert_eq!(
                part1(example),
                *expected,
            );
        }
    }
}
