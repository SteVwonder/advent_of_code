use std::env;

type Grid = Vec<Vec<char>>;

fn is_mirrored_at_idx(line: &Vec<char>, idx: usize) -> bool {
    let (a, b) = line.split_at(idx);
    a.iter().rev().zip(b).all(|(a,b)| a == b)
}

fn find_vertical_mirror_line_idx(grid: &Grid) -> Option<usize> {
    let mut eligible_mirror_indices: Vec<usize> = Vec::from_iter(1..(grid[0].len()));
    for line in grid.iter() {
        eligible_mirror_indices = eligible_mirror_indices
            .into_iter()
            .filter(|idx| is_mirrored_at_idx(&line, *idx))
            .collect::<Vec<usize>>();
    }
    if eligible_mirror_indices.len() == 0 {
        return None;
    }
    assert!(eligible_mirror_indices.len() == 1);
    Some(eligible_mirror_indices[0])
}

fn rotate_grid(grid: &Grid) -> Grid {
    (0..(grid[0].len())).map(|col_idx| {
        grid.iter().map(|row| row[col_idx]).collect::<Vec<char>>()
    }).collect::<Vec<_>>()
}

fn get_grid_score(grid: &Grid) -> u32 {
    if let Some(idx) = find_vertical_mirror_line_idx(grid) {
        return idx as u32;
    }
    let rotated_grid = rotate_grid(grid);
    if let Some(idx) = find_vertical_mirror_line_idx(&rotated_grid) {
        return idx as u32 * 100;
    }
    unreachable!();
}

fn lines_to_grids(lines: Vec<Vec<char>>) -> Vec<Grid> {
    lines.split(|line| line.len() == 0).map(|grid| grid.to_vec()).collect()
}

fn part1(lines: Vec<Vec<char>>) -> u32 {
    let grids = lines_to_grids(lines);
    grids.iter().map(|grid| { get_grid_score(grid) }).sum()
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
