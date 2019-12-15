use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use itertools::Itertools;

#[derive(Debug,Hash,PartialEq,Eq,Clone,Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, other: Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Segment {
    start: Point, // lower-left most point (x & y are smaller/negative)
    end: Point, // upper-right most point (x & y are larger)
}

impl Segment {
    fn new(a: Point, b: Point) -> Segment {
        if a == b {
            panic!("Points in a segment cannot be equal");
        } else if a.x == b.x {
            if a.y < b.y {
                Segment { start: a, end: b }
            } else if b.y < a.y {
                Segment { start: b, end: a }
            } else {
                panic!("Points cannot be equal");
            }
        } else if a.y == b.y {
            if a.x < b.x {
                Segment { start: a, end: b }
            } else if b.x < a.x {
                Segment { start: b, end: a }
            } else {
                panic!("Points cannot be equal");
            }
        }
        else {
            panic!("Segments cannot be diagonal");
        }
    }
    fn horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    fn vertical(&self) -> bool {
        self.start.x == self.end.x
    }
    fn intersect(&self, other: &Segment) -> Option<Point> {
        fn helper(horizontal: &Segment, vertical: &Segment) -> Option<Point> {
            let vert_x = vertical.start.x;
            let hor_y = horizontal.start.y;
            if horizontal.start.x <= vert_x &&
                horizontal.end.x >= vert_x &&
                vertical.start.y <= hor_y &&
                vertical.end.y >= hor_y {
                    return Some(Point { x: vert_x, y: hor_y });
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
    segments: Vec<Segment>
}

impl Wire {
    fn from_string(line: &String) -> Wire {
        let mut segments = Vec::<Segment>::new();
        let instructions = line.split(",").map(|instruction| {
            let (direction, length_str) = instruction.split_at(1);
            (direction, length_str.parse::<i32>().unwrap())
        });
        let mut curr_pos = Point{x: 0, y: 0};
        for (direction, length) in instructions {
            let new_pos: Point = match direction {
                "U" => Point { x: curr_pos.x, y: curr_pos.y + length },
                "D" => Point { x: curr_pos.x, y: curr_pos.y - length },
                "R" => Point { x: curr_pos.x + length, y: curr_pos.y },
                "L" => Point { x: curr_pos.x - length, y: curr_pos.y },
                _ => panic!("Unknown direction ({})", direction),
            };
            segments.push(Segment::new(curr_pos, new_pos));
            curr_pos = new_pos;
        }
        Wire { segments }
    }
    fn intersect(&self, other: Wire) -> Vec<Point> {
        self.segments.iter().cartesian_product(other.segments.iter()).filter_map(|(a, b)| {
            a.intersect(b)
        }).filter(|point| *point != Point {x:0,y:0}).collect()
    }
}

fn solve(wire1: Wire, wire2: Wire) -> u32 {
    wire1.intersect(wire2).iter().map(|point| {
        point.manhattan_distance(Point { x: 0, y: 0 })
    }).min().unwrap()
}

fn main() {
    let file = File::open(Path::new("./input")).unwrap();
    let reader = BufReader::new(file);
    for chunk in reader.lines().map(|l| l.unwrap()).chunks(2).into_iter() {
        let (wire1, wire2) = match chunk.collect::<Vec<String>>().as_slice() {
            [line1, line2] => (Wire::from_string(line1), Wire::from_string(line2)),
            _ => panic!("Malformed input file"),
        };
        println!("Part 1: {}", solve(wire1, wire2));
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
        let mut last = wire.segments.last().unwrap();
        assert_eq!(last.start, Point { x: 3, y: 2 });
        assert_eq!(last.end, Point { x: 3, y: 5 });

        wire = Wire::from_string(&"U7,R6,D4,L4".to_string());
        assert_eq!(wire.segments[0].start, Point { x: 0, y: 0 });
        assert_eq!(wire.segments[0].end, Point { x: 0, y: 7 });
        last = wire.segments.last().unwrap();
        assert_eq!(last.start, Point { x: 2, y: 3 });
        assert_eq!(last.end, Point { x: 6, y: 3 });
    }

    #[test]
    fn test_segment() {
        let segment1 = Segment::new(Point {x: 0, y: 0}, Point {x: 5, y: 0});
        assert_eq!(segment1.horizontal(), true);
        assert_eq!(segment1.vertical(), false);
        let segment2 = Segment::new(Point {x: 3, y: -2}, Point {x: 3, y: 4});
        assert_eq!(segment2.vertical(), true);
        assert_eq!(segment2.horizontal(), false);
        let mut intersection: Point = segment1.intersect(&segment2).unwrap();
        let expected: Point = Point { x: 3, y: 0 };
        assert_eq!(intersection, expected);

        intersection = segment2.intersect(&segment1).unwrap();
        assert_eq!(intersection, expected);

        let segment3 = Segment::new(Point {x: 4, y: 3}, Point {x: 4, y: 7});
        let result = segment1.intersect(&segment3);
        assert_eq!(result.is_none(), true);
    }

    #[test]
    fn wire_intersect() {
        let wire1 = Wire::from_string(&"R8,U5,L5,D3".to_string());
        let wire2 = Wire::from_string(&"U7,R6,D4,L4".to_string());
        let intersections: HashSet<Point> = wire1.intersect(wire2).into_iter().collect();
        let expected: HashSet<Point> = vec![Point { x: 3, y: 3 }, Point { x: 6, y: 5 }].into_iter().collect();
        assert_eq!(intersections, expected);
    }

    #[test]
    fn try_given_tests() {
        let file = File::open(Path::new("./test")).unwrap();
        let reader = BufReader::new(file);
        for chunk in reader.lines().map(|l| l.unwrap()).chunks(3).into_iter() {
            let (wire1, wire2, answer) = match chunk.collect::<Vec<String>>().as_slice() {
                [line1, line2, line3] => (Wire::from_string(line1), Wire::from_string(line2), line3.parse::<u32>().unwrap()),
                _ => panic!("Malformed input file"),
            };
            println!("solve({:?}, {:?}) == {}", wire1, wire2, answer);
            assert_eq!(solve(wire1, wire2), answer);
        }
    }
}
