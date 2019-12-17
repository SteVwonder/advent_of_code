use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
    fn intersects_segment(&self, other: &Segment) -> bool {
        self.x >= other.lower_left().x
            && self.x <= other.upper_right().x
            && self.y >= other.lower_left().y
            && self.y <= other.upper_right().y
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn new(a: Point, b: Point) -> Segment {
        if a == b {
            panic!("Points in a segment cannot be equal");
        } else if a.x != b.x && a.y != b.y {
            panic!("Segments cannot be diagonal");
        }
        Segment { start: a, end: b }
    }
    // lower-left most point (x & y are smaller/negative)
    fn lower_left(&self) -> Point {
        if self.vertical() {
            if self.start.y < self.end.y {
                self.start
            } else {
                //self.end.y < self.start.y
                self.end
            }
        } else {
            // self.horizontal
            if self.start.x < self.end.x {
                self.start
            } else {
                //self.end.x < self.start.x
                self.end
            }
        }
    }
    // upper-right most point (x & y are larger)
    fn upper_right(&self) -> Point {
        if self.vertical() {
            if self.start.y < self.end.y {
                self.end
            } else {
                //self.end.y < self.start.y
                self.start
            }
        } else {
            // self.horizontal
            if self.start.x < self.end.x {
                self.end
            } else {
                //self.end.x < self.start.x
                self.start
            }
        }
    }
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn length(&self) -> u32 {
        self.start.manhattan_distance(self.end)
    }
    fn intersect(&self, other: &Segment) -> Option<Point> {
        fn helper(horizontal: &Segment, vertical: &Segment) -> Option<Point> {
            let vert_x = vertical.start.x;
            let hor_y = horizontal.start.y;
            if horizontal.lower_left().x <= vert_x
                && horizontal.upper_right().x >= vert_x
                && vertical.lower_left().y <= hor_y
                && vertical.upper_right().y >= hor_y
            {
                return Some(Point {
                    x: vert_x,
                    y: hor_y,
                });
            }
            return None;
        }
        if self.horizontal() {
            if other.horizontal() {
                return None;
            } else {
                return helper(self, other);
            }
        } else if self.vertical() {
            if other.vertical() {
                return None;
            } else {
                return helper(other, self);
            }
        } else {
            panic!("Invalid Segments");
        }
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.end)
    }
}

#[derive(Debug)]
struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    fn from_string(line: &String) -> Wire {
        let mut segments = Vec::<Segment>::new();
        let instructions = line.split(",").map(|instruction| {
            let (direction, length_str) = instruction.split_at(1);
            (direction, length_str.parse::<i32>().unwrap())
        });
        let mut curr_pos = Point { x: 0, y: 0 };
        for (direction, length) in instructions {
            let new_pos: Point = match direction {
                "U" => Point {
                    x: curr_pos.x,
                    y: curr_pos.y + length,
                },
                "D" => Point {
                    x: curr_pos.x,
                    y: curr_pos.y - length,
                },
                "R" => Point {
                    x: curr_pos.x + length,
                    y: curr_pos.y,
                },
                "L" => Point {
                    x: curr_pos.x - length,
                    y: curr_pos.y,
                },
                _ => panic!("Unknown direction ({})", direction),
            };
            segments.push(Segment::new(curr_pos, new_pos));
            curr_pos = new_pos;
        }
        Wire { segments }
    }
    fn intersect(&self, other: &Wire) -> Vec<Point> {
        self.segments
            .iter()
            .cartesian_product(other.segments.iter())
            .filter_map(|(a, b)| a.intersect(b))
            .filter(|point| *point != Point { x: 0, y: 0 })
            .collect()
    }
    fn walk_distance(&self, point: &Point) -> u32 {
        let mut distance = 0;
        for segment in self.segments.iter() {
            if point.intersects_segment(&segment) {
                return distance + point.manhattan_distance(segment.start);
            }
            distance += segment.length();
        }
        panic!("Failed to intersect point on path");
    }
}

fn solve_part1(wire1: &Wire, wire2: &Wire) -> u32 {
    wire1
        .intersect(wire2)
        .iter()
        .map(|point| point.manhattan_distance(Point { x: 0, y: 0 }))
        .min()
        .unwrap()
}

fn solve_part2(wire1: &Wire, wire2: &Wire) -> u32 {
    wire1
        .intersect(wire2)
        .iter()
        .map(|point| wire1.walk_distance(point) + wire2.walk_distance(point))
        .min()
        .unwrap()
}

fn main() {
    let file = File::open(Path::new("./input")).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().filter_map(Result::ok).map(|l| Wire::from_string(&l));
    loop {
        match (lines.next(), lines.next()) {
            (Some(wire1), Some(wire2)) => {
                println!("Part 1: {}", solve_part1(&wire1, &wire2));
                println!("Part 2: {}", solve_part2(&wire1, &wire2));
            },
            (Some(wire1), None) => panic!("Malformed input, last wire: {:?}", wire1),
            (None, None) => break,
            _ => panic!(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn wire_from_string() {
        let mut wire = Wire::from_string(&"R8,U5,L5,D3".to_string());
        assert_eq!(wire.segments[0].start, Point { x: 0, y: 0 });
        assert_eq!(wire.segments[0].end, Point { x: 8, y: 0 });
        assert_eq!(wire.segments[0].length(), 8);
        let mut last = wire.segments.last().unwrap();
        assert_eq!(last.start, Point { x: 3, y: 5 });
        assert_eq!(last.end, Point { x: 3, y: 2 });

        wire = Wire::from_string(&"U7,R6,D4,L4".to_string());
        assert_eq!(wire.segments[0].start, Point { x: 0, y: 0 });
        assert_eq!(wire.segments[0].end, Point { x: 0, y: 7 });
        last = wire.segments.last().unwrap();
        assert_eq!(last.start, Point { x: 6, y: 3 });
        assert_eq!(last.upper_right(), Point { x: 6, y: 3 });
        assert_eq!(last.end, Point { x: 2, y: 3 });
        assert_eq!(last.lower_left(), Point { x: 2, y: 3 });
    }

    #[test]
    fn test_segment() {
        let segment1 = Segment::new(Point { x: 0, y: 0 }, Point { x: 5, y: 0 });
        assert_eq!(segment1.horizontal(), true);
        assert_eq!(segment1.vertical(), false);
        let segment2 = Segment::new(Point { x: 3, y: -2 }, Point { x: 3, y: 4 });
        assert_eq!(segment2.vertical(), true);
        assert_eq!(segment2.horizontal(), false);
        let mut intersection: Point = segment1.intersect(&segment2).unwrap();
        let expected: Point = Point { x: 3, y: 0 };
        assert_eq!(intersection, expected);

        intersection = segment2.intersect(&segment1).unwrap();
        assert_eq!(intersection, expected);

        let segment3 = Segment::new(Point { x: 4, y: 3 }, Point { x: 4, y: 7 });
        let result = segment1.intersect(&segment3);
        assert_eq!(result.is_none(), true);

        assert_eq!(Point { x: 3, y: 0 }.intersects_segment(&segment1), true);
        assert_eq!(Point { x: 3, y: -1 }.intersects_segment(&segment2), true);
        assert_eq!(Point { x: 3, y: -4 }.intersects_segment(&segment2), false);
        assert_eq!(Point { x: 3, y: 5 }.intersects_segment(&segment2), false);
        assert_eq!(Point { x: 3, y: 15 }.intersects_segment(&segment1), false);
        assert_eq!(Point { x: 3, y: 1 }.intersects_segment(&segment1), false);
        assert_eq!(
            Point { x: 6, y: 5 }
                .intersects_segment(&Segment::new(Point { x: 8, y: 5 }, Point { x: 3, y: 5 })),
            true
        );
    }

    #[test]
    fn wire_intersect() {
        let wire1 = Wire::from_string(&"R8,U5,L5,D3".to_string());
        let wire2 = Wire::from_string(&"U7,R6,D4,L4".to_string());
        let intersections: HashSet<Point> = wire1.intersect(&wire2).into_iter().collect();
        let expected: HashSet<Point> = vec![Point { x: 3, y: 3 }, Point { x: 6, y: 5 }]
            .into_iter()
            .collect();
        assert_eq!(intersections, expected);

        assert_eq!(wire1.walk_distance(&Point { x: 6, y: 5 }), 15);
        assert_eq!(wire2.walk_distance(&Point { x: 6, y: 5 }), 15);
    }

    #[test]
    fn try_given_tests() {
        let file = File::open(Path::new("./test")).unwrap();
        let reader = BufReader::new(file);
        for chunk in reader.lines().map(|l| l.unwrap()).chunks(4).into_iter() {
            let (wire1, wire2, answer1, answer2) = match chunk.collect::<Vec<String>>().as_slice() {
                [line1, line2, line3, line4] => (
                    Wire::from_string(line1),
                    Wire::from_string(line2),
                    line3.parse::<u32>().unwrap(),
                    line4.parse::<u32>().unwrap(),
                ),
                _ => panic!("Malformed input file"),
            };
            println!("solve_part1({:?}, {:?}) == {}", wire1, wire2, answer1);
            assert_eq!(solve_part1(&wire1, &wire2), answer1);
            println!("solve_part2({:?}, {:?}) == {}", wire1, wire2, answer2);
            assert_eq!(solve_part2(&wire1, &wire2), answer2);
        }
    }
}
