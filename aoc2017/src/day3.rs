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
        //dbg!(self.id, self.start, self.end);
        //assert!(self.contains(val));
        let offset = val - self.start;
        //dbg!(offset);

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
    if let Some(layer) = (1..).map(Layer::new).find(|l| l.contains(int)) {
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
fn part2(input: &i32) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (input, output) in [(1, 0), (12, 3), (23, 2), (28, 3), (41, 4), (1024, 31)] {
            assert_eq!(part1(&input), output);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
