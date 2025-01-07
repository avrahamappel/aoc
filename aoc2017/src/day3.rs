use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> i32 {
    input.parse().unwrap_or(0)
}

/// Determine position of integer within grid
/// Algorithm by claude
fn grid_pos(value: i32) -> (i32, i32) {
    let mut layer = 0;
    let mut start = 1;
    while start + 8 * layer <= value {
        start += 8 * layer;
        layer += 1;
    }
    //dbg!(value, layer, start);

    let offset = value - start + 1;
    //dbg!(offset);
    let (x, y) = if (0..layer * 2).contains(&offset) {
        (layer, layer - offset)
    } else if (layer * 2..layer * 4).contains(&offset) {
        (layer - (offset - layer * 2), -layer)
    } else if (layer * 4..=layer * 6).contains(&offset) {
        (-layer, -layer + (offset - layer * 4))
    } else {
        (-layer + (offset - layer * 6), layer)
    };

    (x, y)
}

#[aoc(day3, part1)]
fn part1(input: &i32) -> i32 {
    if *input <= 1 {
        return 0;
    }
    // determine coords of input cell
    let (x, y) = grid_pos(*input - 1);
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
    fn test_grid_pos() {
        for (input, output) in [
            (1, (1, 0)),
            (2, (1, -1)),
            (3, (0, -1)),
            (4, (-1, -1)),
            (5, (-1, 0)),
            (6, (-1, 1)),
            (7, (0, 1)),
            (8, (1, 1)),
            (9, (2, 1)),
            (10, (2, 0)),
            (11, (2, -1)),
            (12, (2, -2)),
            (13, (1, -2)),
            (14, (0, -2)),
            (15, (-1, -2)),
            (16, (-2, -2)),
            (17, (-2, -1)),
            (18, (-2, 0)),
            (19, (-2, 1)),
            (20, (-2, 2)),
            (21, (-1, 2)),
            (22, (0, 2)),
            (23, (1, 2)),
            (24, (2, 2)),
            (25, (3, 2)),
        ] {
            assert_eq!(output, grid_pos(input));
        }
    }

    #[test]
    fn part1_example() {
        for (input, output) in [(1, 0), (12, 3), (23, 2), (1024, 31)] {
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
