use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    dir: Direction,
    len: u32,
}

type Wire = Vec<Vector>;
type Position = (i32, i32);

fn wire_positions(wire: &Wire) -> Vec<Position> {
    let mut positions = vec![];
    let mut pos = (0, 0);
    for v in wire {
        //dbg!(v);
        // Add all positions to vec
        for _ in 0..v.len {
            let (x, y) = pos;
            pos = match v.dir {
                Direction::Up => (x, y + 1),
                Direction::Down => (x, y - 1),
                Direction::Right => (x + 1, y),
                Direction::Left => (x - 1, y),
            };
            //dbg!(pos);
            positions.push(pos);
        }
    }
    positions
}

struct Input {
    wire_a: Wire,
    wire_b: Wire,
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    let mut wires = input.trim().lines().map(|l| {
        l.trim()
            .split(',')
            .map(|v_str| {
                let dir = match v_str.chars().next().unwrap() {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    _ => unimplemented!(),
                };

                let len = v_str[1..].parse().unwrap();

                Vector { dir, len }
            })
            .collect()
    });

    let wire_a = wires.next().unwrap();
    let wire_b = wires.next().unwrap();

    Input { wire_a, wire_b }
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> i32 {
    // get all wire a's positions
    let a_ps = wire_positions(&input.wire_a)
        .into_iter()
        .collect::<HashSet<_>>();
    //dbg!(&a_ps);
    let mut nearest = i32::MAX;
    for pos in wire_positions(&input.wire_b) {
        //dbg!(pos);
        if a_ps.contains(&pos) {
            let (x, y) = pos;
            nearest = nearest.min(x.abs() + y.abs());
            //dbg!(nearest);
        }
    }
    nearest
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (input, output) in [
            (
                "R8,U5,L5,D3
                 U7,R6,D4,L4",
                6,
            ),
            (
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
                 U62,R66,U55,R34,D71,R55,D58,R83",
                159,
            ),
            (
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
                 U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
                135,
            ),
        ] {
            assert_eq!(part1(&parse(input)), output);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
