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

#[derive(Clone, PartialEq)]
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
    fn is_occupied(&self, row: i32, col: i32) -> u8 {
        if col >= self.width as i32 || row >= self.height as i32 || row < 0 || col < 0 {
            return 0;
        }
        (*self
            .data
            .get(row as usize)
            .unwrap()
            .get(col as usize)
            .unwrap()
            == Seat::Occupied) as u8
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

fn count_neighbors(map: &Map, row: i32, col: i32) -> u8 {
    map.is_occupied(row - 1, col)
        + map.is_occupied(row + 1, col)
        + map.is_occupied(row - 1, col - 1)
        + map.is_occupied(row, col - 1)
        + map.is_occupied(row + 1, col - 1)
        + map.is_occupied(row - 1, col + 1)
        + map.is_occupied(row, col + 1)
        + map.is_occupied(row + 1, col + 1)
}

fn evolve(map: &Map, new_map: &mut Map) {
    new_map.clone_from(map);

    for (row_idx, row) in map.data.iter().enumerate() {
        for (col_idx, value) in row.iter().enumerate() {
            if *value == Seat::Floor {
                continue;
            }
            let num_neighbors = count_neighbors(map, row_idx as i32, col_idx as i32);
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

fn part1(input: &str) -> Result<usize> {
    let mut map = get_map(input)?;
    let mut new_map = map.clone();

    //println!("{}", new_map);
    loop {
        evolve(&map, &mut new_map);
        if map == new_map {
            break;
        }
        //println!("{}", new_map);
        swap(&mut map, &mut new_map);
    }

    Ok(map.data.iter().flatten().filter(|&x| *x == Seat::Occupied).count())
}

fn part2(input: &str) -> Result<u32> {
    Ok(0)
}
