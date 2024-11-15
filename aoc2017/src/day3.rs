use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> i32 {
    input.parse().unwrap_or(0)
}

#[derive(Clone, Copy)]
struct Layer {
    id: i32,
    start: i32,
    end: i32,
}

impl Layer {
    fn new(id: i32) -> Self {
        let sq = |n| n * n;
        let start = sq(((id - 1) * 2) + 1) + 1;
        let end = sq((id * 2) + 1);
        //dbg!(id, start, end);
        Self { id, start, end }
    }

    fn contains(self, val: i32) -> bool {
        self.start <= val && val <= self.end
    }

    fn pos_of(self, val: i32) -> (i32, i32) {
        dbg!(self.id, self.start, self.end);
        //assert!(self.contains(val));
        let offset = val - self.start;
        dbg!(offset);

        // calculate corners
        let k = self.id;
        let o = offset + 1;

        match o / k {
            0 => (k, o % k),
            1 => (k, -(o % k)),
            2 => (o % k, -k),
            3 => (-(o % k), -k),
            4 => (-k, -(o % k)),
            5 => (-k, o % k),
            6 => (-(o % k), k),
            7 => (o % k, k),
            _ => unreachable!(),
        }
    }
}

/// Determine position of integer within grid
/// Algorithm by chatgpt
fn grid_pos(int: i32) -> (i32, i32) {
    // Determine the "layer" around the starting point
    if let Some(layer) = (1..i32::MAX).map(Layer::new).find(|l| l.contains(int)) {
        layer.pos_of(int)
    } else {
        unimplemented!()
    }
}

#[aoc(day3, part1)]
fn part1(input: &i32) -> i32 {
    if *input <= 1 {
        return 0;
    }
    // determine coords of input cell
    let (x, y) = grid_pos(*input);
    dbg!(x, y);
    // calculate distance to 0,0
    x.abs() + y.abs()
}

#[aoc(day3, part2)]
fn part2(input: &i32) -> i32 {
    let mut grid = HashMap::from([((0, 0), 1)]);
    for int in 2..i32::MAX {
        //eprintln!();
        //eprintln!("---DEBUG---");
        //dbg!(int);
        // Generate grid, populating each cell with the sum of populated neighbors
        let pos = grid_pos(int);
        //dbg!(pos);
        let get_neighbor_coords = |(x, y)| {
            [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ]
        };
        let neighbors = get_neighbor_coords(pos);
        let value = grid
            .iter()
            .filter(|(p, _)| (neighbors.contains(p)))
            .map(|(_, v)| v)
            .sum();
        //dbg!(value);

        // When the value is higher than `input`, return the value
        if value > *input {
            return value;
        }

        grid.insert(pos, value);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (input, output) in [
            (1, 0),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 2),
            (6, 1),
            (7, 2),
            (8, 1),
            (9, 2),
            (12, 3),
            (23, 2),
            (28, 3),
            (41, 4),
            (1024, 31),
        ] {
            assert_eq!(part1(&input), output);
        }
    }

    #[test]
    fn part2_example() {
        for (input, output) in [(3, 4), (6, 10), (12, 23), (24, 25)] {
            assert_eq!(part2(&input), output);
        }
    }
}
