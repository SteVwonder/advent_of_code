#![feature(half_open_range_patterns)]

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

use num_traits::FromPrimitive;

use std::io::{self, Read};

use anyhow::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let part1_res = part1(&input)?;
    println!("Part 1: {}", part1_res);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}

#[derive(Primitive, Clone, Debug, PartialEq, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

enum RelDirection {
    Left,
    Right,
}

struct Ship {
    position: (i32, i32),
    direction: Direction,
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: (0, 0),
            direction: Direction::East,
        }
    }

    fn r#move(&mut self, dir: Direction, mag: u32) {
        match dir {
            Direction::North => self.position.1 += mag as i32,
            Direction::South => self.position.1 -= mag as i32,
            Direction::East => self.position.0 += mag as i32,
            Direction::West => self.position.0 -= mag as i32,
        };
    }

    fn rotate(&mut self, dir: RelDirection, deg: u32) {
        let num_turns = deg as i32 / 90;
        let new_direction = modulo(
            (self.direction as i32)
                + match dir {
                    RelDirection::Left => -1 * num_turns,
                    RelDirection::Right => num_turns,
                },
            4,
        );
        self.direction = Direction::from_i32(new_direction).unwrap();
    }
}

fn part1(input: &str) -> Result<u32> {
    Ok(0)
}

fn part2(input: &str) -> Result<u64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_rotate() {
        let mut ship = Ship::new();
        assert_eq!(ship.direction, Direction::East);
        ship.rotate(RelDirection::Left, 270);
        assert_eq!(ship.direction, Direction::South);
        ship.rotate(RelDirection::Right, 90);
        assert_eq!(ship.direction, Direction::West);
        ship.rotate(RelDirection::Right, 180);
        assert_eq!(ship.direction, Direction::East);
    }
}
