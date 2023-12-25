use std::collections::HashSet;
use std::env;
use std::fmt;

use ndarray::{Array2, ArrayView2};

#[derive(Clone,Debug)]
#[repr(u8)]
enum GridElement {
    Empty = 0,
    LLUR,
    ULLR,
    Vertical,
    Horizontal,
}

#[derive(PartialEq,Eq,Hash,Clone,Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

type Grid = Array2<GridElement>;
type GridView<'a> = ArrayView2<'a, GridElement>;

type InputType<'a> = &'a Vec<Vec<char>>;
type Index = (usize, usize);

#[derive(PartialEq,Eq,Clone,Hash)]
struct Path {
    index: Index,
    dir: Dir,
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path [({} {}), {:#?}]", self.index.0, self.index.1, self.dir)
    }
}

fn move_forward(curr_position: &Index, direction: &Dir) -> Option<Index> {
    let out = match direction {
        Dir::North => (curr_position.0.checked_sub(1), Some(curr_position.1)),
        Dir::South => (curr_position.0.checked_add(1), Some(curr_position.1)),
        Dir::East => (Some(curr_position.0), curr_position.1.checked_add(1)),
        Dir::West => (Some(curr_position.0), curr_position.1.checked_sub(1)),
    };
    match out {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None,
    }
}

fn get_next_paths(grid: GridView, curr_path: Path) -> Vec<Path> {
    let curr_position = curr_path.index;
    let curr_direction = curr_path.dir;
    let mut paths = Vec::new();
    if let Some(curr_value) = grid.get(curr_position) {
        match curr_value {
            GridElement::Empty => {
                move_forward(&curr_position, &curr_direction)
                    .map(|index| paths.push(Path{index, dir: curr_direction}));
            }
            GridElement::LLUR => {
                let dir = match curr_direction {
                    Dir::North => Dir::East,
                    Dir::South => Dir::West,
                    Dir::East => Dir::North,
                    Dir::West => Dir::South,
                };
                move_forward(&curr_position, &dir)
                        .map(|index| paths.push(Path{index, dir}));
            },
            GridElement::ULLR => {
                let dir = match curr_direction {
                    Dir::North => Dir::West,
                    Dir::South => Dir::East,
                    Dir::East => Dir::South,
                    Dir::West => Dir::North,
                };
                move_forward(&curr_position, &dir)
                        .map(|index| paths.push(Path{index, dir}));
            },
            GridElement::Vertical => match curr_direction {
                Dir::North | Dir::South => {
                    move_forward(&curr_position, &curr_direction)
                        .map(|index| paths.push(Path{index, dir: curr_direction}));
                }
                Dir::East | Dir::West => {
                    for dir in vec![Dir::North, Dir::South].into_iter() {
                        move_forward(&curr_position, &dir).map(|index| paths.push(Path{index, dir}));
                    }
                }
            },
            GridElement::Horizontal => match curr_direction {
                Dir::North | Dir::South => {
                    for dir in vec![Dir::East, Dir::West].into_iter() {
                        move_forward(&curr_position, &dir).map(|index| paths.push(Path{index, dir}));
                    }
                },
                Dir::East | Dir::West => {
                    move_forward(&curr_position, &curr_direction)
                        .map(|index| paths.push(Path{index, dir: curr_direction}));
                },
            },
        };
    }
    paths
}

fn propagate_light(grid: GridView, starting_path: Path) -> Array2<u32> {
    let mut energized = Array2::<u32>::zeros(grid.raw_dim());
    let mut paths: Vec<Path> = vec![starting_path];
    let mut prev_paths = HashSet::<Path>::new();

    while let Some(path) = paths.pop() {
        prev_paths.insert(path.clone());
        if let Some(curr_energized) = energized.get_mut(path.index) {
            *curr_energized = 1;
            let new_paths = get_next_paths(grid, path);
            let filtered_paths = new_paths.into_iter().filter(|x| !prev_paths.contains(x)).collect::<Vec<_>>();
            paths.extend(filtered_paths.into_iter());
        }
    }

    energized
}

fn part1(lines: InputType) -> u32 {
    let grid = lines_to_grid(lines);
    let energized = propagate_light(grid.view(), Path{index: (0, 0), dir: Dir::East});
    energized.sum()
}

fn part2(lines: InputType) -> u32 {
    let grid = lines_to_grid(lines);
    let num_rows = grid.shape()[0];
    let num_cols = grid.shape()[1];
    let vert = (0..num_cols).flat_map(|x| vec![
        Path{index: (0, x), dir: Dir::South},
        Path{index: (num_rows-1, x), dir: Dir::North},
    ]);
    let hori = (0..num_rows).flat_map(|x| vec![
        Path{index: (x, 0), dir: Dir::East},
        Path{index: (x, num_cols-1), dir: Dir::West},
    ]);
    let view = grid.view();
    vert.chain(hori).map(|x| propagate_light(view, x).sum()).max().unwrap()
}

fn lines_to_grid(lines: &Vec<Vec<char>>) -> Grid {
    let data = lines
        .iter()
        .flat_map(|line: &Vec<char>| {
            line.iter().map(|c: &char| {
                (match c {
                    '.' => GridElement::Empty,
                    '/' => GridElement::LLUR,
                    '\\' => GridElement::ULLR,
                    '|' => GridElement::Vertical,
                    '-' => GridElement::Horizontal,
                    _ => unreachable!("{}", c),
                } as GridElement)
            })
        })
        .collect::<Vec<GridElement>>();
    let shape = (lines.len(), lines[0].len());
    Array2::from_shape_vec(shape, data).unwrap()
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
    println!("Part2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    type Example = Vec<Vec<char>>;

    fn get_part1_examples() -> Vec<Example> {
        let examples = vec![vec![
            r".|...\....",
            r"|.-.\.....",
            r".....|-...",
            r"........|.",
            r"..........",
            r".........\",
            r"..../.\\..",
            r".-.-/..|..",
            r".|....-|.\",
            r"..//.|....",
        ]];
        examples
            .into_iter()
            .map(|example| {
                example
                    .iter()
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect()
            })
            .collect()
    }

    #[test]
    fn test_move_forward() {
        assert_eq!(
            move_forward(&(0,0), &Dir::East),
            Some((0,1)),
        );
        assert_eq!(
            move_forward(&(0,0), &Dir::South),
            Some((1,0)),
        );
        assert_eq!(
            move_forward(&(0,0), &Dir::West),
            None,
        );
        assert_eq!(
            move_forward(&(0,0), &Dir::North),
            None,
        );
    }


    #[test]
    fn test_next_path() {
        let example = &get_part1_examples()[0];
        let grid = lines_to_grid(example);
        let grid_view = grid.view();
        let next_paths = get_next_paths(grid_view, Path{index: (0,0), dir: Dir::East});
        assert_eq!(next_paths, vec![Path{index: (0,1), dir: Dir::East}]);
    }

    #[test]
    fn test_part1_e2e() {
        let examples = get_part1_examples();
        let expected = vec![46];
        for (example, expected) in examples.into_iter().zip(expected.into_iter()) {
            assert_eq!(part1(&example), expected);
        }
    }

    #[test]
    fn test_part2_e2e() {
        let examples = get_part1_examples();
        let expected = vec![51];
        for (example, expected) in examples.into_iter().zip(expected.into_iter()) {
            assert_eq!(part2(&example), expected);
        }
    }
}
