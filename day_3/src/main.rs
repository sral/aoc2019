// --- Day 3: Crossed Wires ---
//
// The gravity assist was successful, and you're well on your way to the Venus
// refuelling station. During the rush back on Earth, the fuel management system
// wasn't completely installed, so that's next on the priority list.
//
// Opening the front panel reveals a jumble of wires. Specifically, two wires
// are connected to a central port and extend outward on a grid. You trace the
// path each wire takes as it leaves the central port, one wire per line of text
// (your puzzle input).
//
// The wires twist and turn, but the two wires occasionally cross paths. To fix
// the circuit, you need to find the intersection point closest to the central
// port. Because the wires are on a grid, use the Manhattan distance for this
// measurement. While the wires do technically cross right at the central port
// where they both start, this point does not count, nor does a wire count as
// crossing with itself.
//
// For example, if the first wire's path is R8,U5,L5,D3, then starting from the
// central port (o), it goes right 8, up 5, left 5, and finally down 3:
//
// ...........
// ...........
// ...........
// ....+----+.
// ....|....|.
// ....|....|.
// ....|....|.
// .........|.
// .o-------+.
// ...........
//
// Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down
// 4, and left 4:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// These wires cross at two locations (marked X), but the lower-left one is
// closer to the central port: its distance is 3 + 3 = 6.
//
// Here are a few more examples:
//
// - R75,D30,R83,U83,L12,D49,R71,U7,L72 U62,R66,U55,R34,D71,R55,D58,R83 =
//   distance 159
// - R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//
// What is the Manhattan distance from the central port to the closest
// intersection?
//
// Your puzzle answer was 860.
//
// The first half of this puzzle is complete! It provides one gold star: *
//
// --- Part Two ---
//
// It turns out that this circuit is very timing-sensitive; you actually need to
// minimize the signal delay.
//
// To do this, calculate the number of steps each wire takes to reach each
// intersection; choose the intersection where the sum of both wires' steps is
// lowest. If a wire visits a position on the grid multiple times, use the steps
// value from the first time it visits that position when calculating the total
// value of a specific intersection.
//
// The number of steps a wire takes is the total number of grid squares the wire
// has entered to get to that location, including the intersection being
// considered. Again consider the example from above:
//
// ...........
// .+-----+...
// .|.....|...
// .|..+--X-+.
// .|..|..|.|.
// .|.-X--+.|.
// .|..|....|.
// .|.......|.
// .o-------+.
// ...........
//
// In the above example, the intersection closest to the central port is reached
// after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the
// second wire for a total of 20+20 = 40 steps.
//
// However, the top-right intersection is better: the first wire takes only
// 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30
// steps.
//
// Here are the best steps for the extra examples from above:
//
// - R75,D30,R83,U83,L12,D49,R71,U7,L72
//   U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
// - R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//
// What is the fewest combined steps the wires must take to reach an intersection?
//
// Your puzzle answer was 9238.
//
// Both parts of this puzzle are complete! They provide two gold stars: **

#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::collections::{HashMap, HashSet};
use std::io;
use std::io::prelude::*;

use std::ops::Add;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Vec2d {
    x: i32,
    y: i32,
}

impl Vec2d {
    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Add for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Self {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl From<&str> for Vec2d {
    fn from(s: &str) -> Vec2d {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([UDLR])(\d+$)").unwrap();
        }
        let captures = RE.captures(s).unwrap();
        let direction = &captures[1];
        let magnitude = captures[2].parse::<i32>().unwrap();
        match direction {
            "U" => Vec2d { x: 0, y: magnitude },
            "D" => Vec2d {
                x: 0,
                y: -magnitude,
            },
            "L" => Vec2d {
                x: -magnitude,
                y: 0,
            },
            "R" => Vec2d { x: magnitude, y: 0 },
            _ => panic!(""),
        }
    }
}

fn parse(path: &str) -> Vec<Vec2d> {
    let mut points: Vec<Vec2d> = Vec::new();
    for p in path.split(',') {
        points.push(Vec2d::from(p));
    }
    points
}

fn get_points(vertices: &[Vec2d]) -> Vec<Vec2d> {
    let mut points = Vec::new();
    let mut pos = Vec2d { x: 0, y: 0 };
    for vertex in vertices.iter() {
        let mut direction = Vec2d { x: 0, y: 0 };
        if vertex.x > 0 {
            direction.x = 1;
        } else if vertex.x < 0 {
            direction.x = -1;
        } else if vertex.y > 0 {
            direction.y = 1;
        } else if vertex.y < 0 {
            direction.y = -1;
        }
        // We have no diagonal lines so one of the operands is always 0.
        let line_length = vertex.x.abs() + vertex.y.abs();

        for _ in 0..line_length {
            pos = pos + direction;
            points.push(pos);
        }
    }
    points
}

fn main() {
    let mut wires: Vec<Vec<Vec2d>> = Vec::new();

    for line in io::stdin().lock().lines() {
        match line {
            Ok(l) => wires.push(get_points(&parse(&l))),
            Err(_) => panic!(),
        }
    }

    let wire_one: HashSet<Vec2d> = wires[0].iter().cloned().collect();
    let wire_two: HashSet<Vec2d> = wires[1].iter().cloned().collect();
    let intersections: HashSet<&Vec2d> = wire_one.intersection(&wire_two).collect();

    println!(
        "Part 1: distance: {:?}",
        intersections
            .iter()
            .map(|p| p.manhattan_distance())
            .min()
            .unwrap()
    );

    let mut steps: HashMap<&Vec2d, Vec<usize>> = HashMap::new();
    for wire in wires.iter() {
        for (step, point) in (1..).zip(wire.iter()) {
            if intersections.contains(&point) {
                match steps.get_mut(&point) {
                    Some(v) => v.push(step),
                    None => {
                        steps.insert(&point, vec![step]);
                    }
                }
            }
        }
    }
    println!(
        "Part 2: steps: {:?}",
        steps
            .values()
            .map(|v| v.iter().sum::<usize>())
            .min()
            .unwrap()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_from_str_test() {
        let test_cases = [
            ("R8", Vec2d { x: 8, y: 0 }),
            ("U5", Vec2d { x: 0, y: 5 }),
            ("L5", Vec2d { x: -5, y: 0 }),
            ("D3", Vec2d { x: 0, y: -3 }),
        ];
        for case in test_cases.iter() {
            assert_eq!(Vec2d::from(case.0), case.1);
        }
    }

    #[test]
    fn add_points_test() {
        let a = Vec2d { x: 7, y: 3 };
        let b = Vec2d { x: 3, y: 7 };
        let expected = Vec2d { x: 10, y: 10 };

        assert_eq!(a + b, expected);
    }

    #[test]
    fn parse_path_test() {
        assert_eq!(
            parse("R8,U5,L5,D3"),
            vec![
                Vec2d { x: 8, y: 0 },
                Vec2d { x: 0, y: 5 },
                Vec2d { x: -5, y: 0 },
                Vec2d { x: 0, y: -3 }
            ]
        );
    }

    #[test]
    fn get_points_test() {
        assert_eq!(
            get_points(&parse("R8,U5,L5,D3")),
            vec![
                Vec2d { x: 1, y: 0 },
                Vec2d { x: 2, y: 0 },
                Vec2d { x: 3, y: 0 },
                Vec2d { x: 4, y: 0 },
                Vec2d { x: 5, y: 0 },
                Vec2d { x: 6, y: 0 },
                Vec2d { x: 7, y: 0 },
                Vec2d { x: 8, y: 0 },
                Vec2d { x: 8, y: 1 },
                Vec2d { x: 8, y: 2 },
                Vec2d { x: 8, y: 3 },
                Vec2d { x: 8, y: 4 },
                Vec2d { x: 8, y: 5 },
                Vec2d { x: 7, y: 5 },
                Vec2d { x: 6, y: 5 },
                Vec2d { x: 5, y: 5 },
                Vec2d { x: 4, y: 5 },
                Vec2d { x: 3, y: 5 },
                Vec2d { x: 3, y: 4 },
                Vec2d { x: 3, y: 3 },
                Vec2d { x: 3, y: 2 }
            ]
        );
    }
}
