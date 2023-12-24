use std::collections::{HashMap, VecDeque};
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
    for _ in 0..lines.len() {
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

fn rotate_grid<T>(grid: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let num_cols = grid[0].len();
    let num_rows = grid.len();
    (0..num_cols)
        .map(|row_idx| {
            (0..num_rows)
                .rev()
                .map(|col_idx| grid[col_idx][row_idx].clone())
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

fn grid_key(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<String>()
}

fn part1(lines: &Vec<Vec<char>>) -> u32 {
    let new_lines = move_rocks(lines);
    calc_weight(&new_lines)
}

fn cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // North
    let mut curr_state = move_rocks(grid);
    // West
    curr_state = rotate_grid(&curr_state);
    curr_state = move_rocks(&curr_state);
    // South
    curr_state = rotate_grid(&curr_state);
    curr_state = move_rocks(&curr_state);
    // East
    curr_state = rotate_grid(&curr_state);
    curr_state = move_rocks(&curr_state);

    // North
    curr_state = rotate_grid(&curr_state);

    curr_state
}

fn part2(lines: Vec<Vec<char>>) -> u32 {
    let mut curr_cycle = 0;
    let mut prev_seen = HashMap::<String,u32>::new();
    let mut curr_state = lines;
    loop {
        let key = grid_key(&curr_state);
        if prev_seen.contains_key(&key) {
            break;
        }
        prev_seen.insert(key, curr_cycle);
        curr_state = cycle(&curr_state);
        curr_cycle += 1;
    }
    let cycle_size = curr_cycle - prev_seen.get(&grid_key(&curr_state)).unwrap();
    let cycles_to_do = (1000000000 - curr_cycle) % cycle_size;
    for _ in 0..(cycles_to_do) {
        curr_state = cycle(&curr_state);
    }
    calc_weight(&curr_state)
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

    fn get_part2_expected() -> Vec<Example> {
        let expecteds = vec![
            vec![
                ".....#....",
                "....#...O#",
                "...OO##...",
                ".OO#......",
                ".....OOO#.",
                ".O#...O#.#",
                "....O#....",
                "......OOOO",
                "#...O###..",
                "#..OO#....",
            ],
            vec![
                ".....#....",
                "....#...O#",
                ".....##...",
                "..O#......",
                ".....OOO#.",
                ".O#...O#.#",
                "....O#...O",
                ".......OOO",
                "#..OO###..",
                "#.OOO#...O",
            ],
            vec![
                ".....#....",
                "....#...O#",
                ".....##...",
                "..O#......",
                ".....OOO#.",
                ".O#...O#.#",
                "....O#...O",
                ".......OOO",
                "#...O###.O",
                "#.OOO#...O",
            ],
        ];
        expecteds
            .into_iter()
            .map(|expected| {
                expected
                    .into_iter()
                    .map(|line: &str| (*line).chars().collect::<Vec<char>>())
                    .collect::<Example>()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_rotate_grid() {
        let grid = vec![
            vec!["11", "12", "13"],
            vec!["21", "22", "23"],
            vec!["31", "32", "33"],
        ];
        let expected = vec![
            vec!["31", "21", "11"],
            vec!["32", "22", "12"],
            vec!["33", "23", "13"],
        ];
        let rotated = rotate_grid(&grid);
        assert_eq!(rotated, expected);
        assert_eq!(rotate_grid(&rotate_grid(&rotate_grid(&rotated))), grid);
    }

    #[test]
    fn test_cycle() {
        let examples = get_part1_examples();
        let expected = get_part2_expected();
        let mut curr_state = examples[0].clone();
        for expect in expected.into_iter() {
            curr_state = cycle(&curr_state);
            assert_eq!(curr_state, expect);
        }
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
            assert_eq!(part1(example), *expected,);
        }
    }

    #[test]
    fn test_part2_e2e() {
        let examples = get_part1_examples();
        assert_eq!(part2(examples[0].clone()), 64);
    }
}
