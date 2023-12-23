use std::env;
use std::ops::Sub;

use ndarray::{array, s, Array2, ArrayView2};

type Grid = Array2<i16>;
type GridView<'a> = ArrayView2<'a, i16>;

fn vertically_mirror_at_idx_and_sub(grid: &GridView, idx: usize) -> Grid {
    let num_cols = grid.shape()[1];
    if idx <= 0 || idx >= num_cols {
        panic!();
    }
    let mut left_slice = s![.., 0..idx;-1];
    let mut right_slice = s![.., idx..(2 * idx)];
    if idx > (num_cols / 2) {
        let width = num_cols - idx;
        let buffer = idx - width;
        left_slice = s![.., buffer..idx;-1];
        right_slice = s![.., idx..];
    }
    let left = grid.slice(left_slice);
    let right = grid.slice(right_slice);

    left.sub(&right)
}

fn is_vertically_mirrored_at_idx(grid: &GridView, idx: usize) -> bool {
    vertically_mirror_at_idx_and_sub(grid, idx).iter().all(|x| *x == 0)
}

fn find_vertical_mirror_line_idx(grid: &GridView) -> Option<usize> {
    for idx in 1..(grid.shape()[1]) {
        if is_vertically_mirrored_at_idx(grid, idx) {
            return Some(idx);
        }
    }
    None
}

fn get_grid_score(grid: &GridView) -> u32 {
    if let Some(idx) = find_vertical_mirror_line_idx(grid) {
        return idx as u32;
    }
    if let Some(idx) = find_vertical_mirror_line_idx(&grid.t()) {
        return idx as u32 * 100;
    }
    unreachable!();
}

fn lines_to_grid(lines: &Vec<Vec<char>>) -> Grid {
    let data = lines
        .iter()
        .flat_map(|line: &Vec<char>| {
            line.iter().map(|c: &char| {
                (match c {
                    '.' => 1,
                    '#' => 3,
                    _ => unreachable!(),
                } as i16)
            })
        })
        .collect::<Vec<i16>>();
    let shape = (lines.len(), lines[0].len());
    Array2::from_shape_vec(shape, data).unwrap()
}

fn lines_to_grids(lines: Vec<Vec<char>>) -> Vec<Grid> {
    lines
        .split(|line| line.len() == 0)
        .map(|lines| lines_to_grid(&lines.to_vec()))
        .collect()
}

fn part1(lines: Vec<Vec<char>>) -> u32 {
    let grids = lines_to_grids(lines);
    grids
        .iter()
        .map(|grid: &Grid| get_grid_score(&grid.view()))
        .sum()
}

fn has_smudge_vertically_mirrored_at_idx(grid: &GridView, idx: usize) -> bool {
    let diff = vertically_mirror_at_idx_and_sub(grid, idx);
    diff.iter().enumerate().filter_map(|(idx, elem)| {
     if *elem != 0 {
         return Some(idx);
     }
     None
    }).count() == 1
}

fn find_vertical_mirror_line_smudged_idx(grid: &GridView) -> Option<usize> {
    for idx in 1..(grid.shape()[1]) {
        if has_smudge_vertically_mirrored_at_idx(grid, idx) {
            return Some(idx);
        }
    }
    None
}
fn get_grid_score2(grid: &GridView) -> u32 {
    if let Some(idx) = find_vertical_mirror_line_smudged_idx(grid) {
        return idx as u32;
    }
    if let Some(idx) = find_vertical_mirror_line_smudged_idx(&grid.t()) {
        return idx as u32 * 100;
    }
    println!("{}", grid);
    unreachable!();
}

fn part2(lines: Vec<Vec<char>>) -> u32 {
    let grids = lines_to_grids(lines);
    grids
        .iter()
        .map(|grid: &Grid| get_grid_score2(&grid.view()))
        .sum()
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
    println!("Part1: {}", part1(lines.clone()));
    println!("Part2: {}", part2(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    type Example = Vec<Vec<char>>;

    fn get_part1_examples() -> Vec<Example> {
        let examples = vec![
            vec![
                "#.##..##.",
                "..#.##.#.",
                "##......#",
                "##......#",
                "..#.##.#.",
                "..##..##.",
                "#.#.##.#.",
            ],
            vec![
                "#...##..#",
                "#....#..#",
                "..##..###",
                "#####.##.",
                "#####.##.",
                "..##..###",
                "#....#..#",
            ],
            vec![
                "#.##..##.",
                "..#.##.#.",
                "##......#",
                "##......#",
                "..#.##.#.",
                "..##..##.",
                "#.#.##.#.",
                "",
                "#...##..#",
                "#....#..#",
                "..##..###",
                "#####.##.",
                "#####.##.",
                "..##..###",
                "#....#..#",
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
    fn build_grid() {
        let examples = get_part1_examples();
        let input = &examples[0];

        let grid = lines_to_grid(&input);
        assert_eq!(
            grid,
            array![
                [3, 1, 3, 3, 1, 1, 3, 3, 1,],
                [1, 1, 3, 1, 3, 3, 1, 3, 1,],
                [3, 3, 1, 1, 1, 1, 1, 1, 3,],
                [3, 3, 1, 1, 1, 1, 1, 1, 3,],
                [1, 1, 3, 1, 3, 3, 1, 3, 1,],
                [1, 1, 3, 3, 1, 1, 3, 3, 1,],
                [3, 1, 3, 1, 3, 3, 1, 3, 1,],
            ]
        );

        let sub: Vec<i16> = vec![3, 1, 1, 1, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected: Vec<i16> = vec![0, 0, 0, 0, 0, 0, 3, 3, 1, 1, 1, 1, 3, 1];

        let subgrid = grid.slice(s![.., ..2]);

        assert_eq!(
            subgrid.sub(Array2::from_shape_vec((7, 2), sub).unwrap()),
            Array2::from_shape_vec((7, 2), expected).unwrap(),
        );
    }

    #[test]
    fn part1_e2e() {
        let examples = get_part1_examples();
        let answers: Vec<u32> = vec![5, 400, 405];
        for (example, answer) in examples.iter().zip(answers.iter()) {
            assert_eq!(part1(example.clone()), *answer);
        }
    }

    #[test]
    fn part2_e2e() {
        let examples = get_part1_examples();
        let answers: Vec<u32> = vec![300, 100, 400];
        for (example, answer) in examples.iter().zip(answers.iter()) {
            assert_eq!(part2(example.clone()), *answer);
        }
    }
}
