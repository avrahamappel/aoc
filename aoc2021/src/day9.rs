use aoc_runner_derive::{aoc, aoc_generator};

type Input = Grid<u32>;

struct Grid<T> {
    line_len: usize,
    vec: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
    /// Get the index of the top left corner of the grid
    #[allow(clippy::unused_self)]
    fn top_left_corner(&self) -> usize {
        0
    }

    /// Get the index of the top right corner of the grid
    fn top_right_corner(&self) -> usize {
        self.line_len - 1
    }

    /// Get the index of the bottom left corner of the grid
    fn bottom_left_corner(&self) -> usize {
        // Calculate the number of rows
        let number_of_rows = self.vec.len() / self.line_len;
        // Calculate the starting index of the last row
        (number_of_rows - 1) * self.line_len
    }

    /// Get the index of the bottom right corner of the grid
    fn bottom_right_corner(&self) -> usize {
        self.vec.len() - 1
    }

    fn get_neighbors(&self, index: usize) -> Vec<T> {
        assert!(index <= self.vec.len());

        // Check if index is along the borders of the grid
        let is_top = index <= self.top_right_corner();
        let is_bottom = index >= self.bottom_left_corner();
        let is_left = index % self.line_len == 0;
        let is_right = (index % self.line_len) == (self.line_len - 1);

        // Add neighbors to vec
        let mut neighbors = Vec::with_capacity(4);
        if !is_top {
            neighbors.push(self.vec[index - self.line_len]);
        }
        if !is_left {
            neighbors.push(self.vec[index - 1]);
        }
        if !is_right {
            neighbors.push(self.vec[index + 1]);
        }
        if !is_bottom {
            neighbors.push(self.vec[index + self.line_len]);
        }

        neighbors
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines().peekable();
    let line_len = lines.peek().unwrap().len();
    let vec = lines
        .flat_map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("All should be digits"))
        })
        .collect();
    Grid { line_len, vec }
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> u32 {
    input
        .vec
        .iter()
        .enumerate()
        .filter(|(i, lvl)| input.get_neighbors(*i).iter().all(|n| n > lvl))
        .map(|(_, lvl)| lvl + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "2199943210
                 3987894921
                 9856789892
                 8767896789
                 9899965678"
            )),
            15
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
