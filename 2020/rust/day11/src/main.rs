#[macro_use]
extern crate lazy_static;

use std::cmp::{max, min};
use std::fmt;
use std::io::{self, Read};
use std::mem::swap;

use anyhow::{anyhow, bail, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Clone, PartialEq, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Seat::Floor => ".",
            Seat::Empty => "L",
            Seat::Occupied => "#",
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Map {
    data: Vec<Vec<Seat>>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(for row in self.data.iter() {
            for col in row {
                write!(f, "{:?}", col)?;
            }
            write!(f, "\n")?;
        })
    }
}

impl Map {
    fn get_value(&self, row: i32, col: i32) -> Option<Seat> {
        if col >= self.width as i32 || row >= self.height as i32 || row < 0 || col < 0 {
            return None;
        }
        Some(
            *self
                .data
                .get(row as usize)
                .unwrap()
                .get(col as usize)
                .unwrap(),
        )
    }

    fn is_occupied(&self, row: i32, col: i32) -> Option<u8> {
        self.get_value(row, col)
            .map(|x| (x == Seat::Occupied) as u8)
    }

    fn dir_is_occupied(&self, pos: (i32, i32), dir: (i32, i32)) -> Option<u8> {
        let mut curr_pos = (pos.0 + dir.0, pos.1 + dir.1);
        //println!("\tDirection {:?}", dir);
        while let Some(value) = self.get_value(curr_pos.0, curr_pos.1) {
            match value {
                Seat::Floor => {}
                Seat::Empty => {
                    //println!("\t\tEmpty");
                    return Some(0);
                }
                Seat::Occupied => {
                    //println!("\t\tOccupied");
                    return Some(1);
                }
            }
            curr_pos.0 += dir.0;
            curr_pos.1 += dir.1;
            //println!("\tNew pos {:?}", curr_pos);
        }
        //println!("\t\tOut Of Bounds");
        None
    }
}

fn get_map(input: &str) -> Result<Map> {
    let mut lines = input.lines().peekable();
    let width = lines.peek().unwrap().chars().count();
    let mut ret = Map {
        data: Vec::new(),
        width,
        height: 0,
    };
    for line in lines {
        let mut row = Vec::with_capacity(width);
        for value in line.chars() {
            let seat = match value {
                '.' => Seat::Floor,
                'L' => Seat::Empty,
                '#' => Seat::Occupied,
                _ => bail!("Invalid seat character"),
            };
            row.push(seat);
        }
        ret.data.push(row);
        ret.height += 1;
    }
    Ok(ret)
}

fn count_neighbors1(map: &Map, row: i32, col: i32) -> u8 {
    map.is_occupied(row - 1, col).unwrap_or(0)
        + map.is_occupied(row + 1, col).unwrap_or(0)
        + map.is_occupied(row - 1, col - 1).unwrap_or(0)
        + map.is_occupied(row, col - 1).unwrap_or(0)
        + map.is_occupied(row + 1, col - 1).unwrap_or(0)
        + map.is_occupied(row - 1, col + 1).unwrap_or(0)
        + map.is_occupied(row, col + 1).unwrap_or(0)
        + map.is_occupied(row + 1, col + 1).unwrap_or(0)
}

fn count_neighbors2(map: &Map, row: i32, col: i32) -> u8 {
    //println!("Position ({}, {})", row, col);
    map.dir_is_occupied((row, col), (-1, 0)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (0, -1)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (0, 1)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (1, 0)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (1, -1)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (-1, 1)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (-1, -1)).unwrap_or(0)
        + map.dir_is_occupied((row, col), (1, 1)).unwrap_or(0)
}

fn evolve1(map: &Map, new_map: &mut Map) {
    new_map.clone_from(map);

    for (row_idx, row) in map.data.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if *value == Seat::Floor {
                continue;
            }
            let num_neighbors = count_neighbors1(map, row_idx as i32, col_idx as i32);
            let new_value = match num_neighbors {
                1..=3 => continue,
                0 => Seat::Occupied,
                4..=8 => Seat::Empty,
                _ => panic!("How did you get 9 neighbors on an 2D surface?"),
            };

            *new_map
                .data
                .get_mut(row_idx)
                .unwrap()
                .get_mut(col_idx)
                .unwrap() = new_value;
        }
    }
}

fn evolve2(map: &Map, new_map: &mut Map) {
    new_map.clone_from(map);

    for (row_idx, row) in map.data.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if *value == Seat::Floor {
                continue;
            }
            let num_neighbors = count_neighbors2(map, row_idx as i32, col_idx as i32);
            let new_value = match num_neighbors {
                1..=4 => continue,
                0 => Seat::Occupied,
                5..=8 => Seat::Empty,
                _ => panic!("How did you get 9 neighbors on an 2D surface?"),
            };

            *new_map
                .data
                .get_mut(row_idx)
                .unwrap()
                .get_mut(col_idx)
                .unwrap() = new_value;
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = get_map(input)?;
    let mut new_map = map.clone();

    //println!("{}", new_map);
    loop {
        evolve1(&map, &mut new_map);
        if map == new_map {
            break;
        }
        //println!("{}", new_map);
        swap(&mut map, &mut new_map);
    }

    Ok(map
        .data
        .iter()
        .flatten()
        .filter(|&x| *x == Seat::Occupied)
        .count())
}

fn part2(input: &str) -> Result<usize> {
    let mut map = get_map(input)?;
    let mut new_map = map.clone();

    //println!("{}", new_map);
    loop {
        evolve2(&map, &mut new_map);
        if map == new_map {
            break;
        }
        //println!("{}", new_map);
        swap(&mut map, &mut new_map);
    }

    Ok(map
        .data
        .iter()
        .flatten()
        .filter(|&x| *x == Seat::Occupied)
        .count())
}
