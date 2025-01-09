use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

type Level = u32;
type Input = Grid<Level>;

struct Grid<T> {
    line_len: usize,
    vec: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone + Copy,
{
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

    fn get_neighbor_positions(&self, index: usize) -> Vec<usize> {
        assert!(index <= self.vec.len());

        // Check if index is along the borders of the grid
        let is_top = index <= self.top_right_corner();
        let is_bottom = index >= self.bottom_left_corner();
        let is_left = index % self.line_len == 0;
        let is_right = (index % self.line_len) == (self.line_len - 1);

        // Add neighbors to vec
        let mut neighbors = Vec::with_capacity(4);
        if !is_top {
            neighbors.push(index - self.line_len);
        }
        if !is_left {
            neighbors.push(index - 1);
        }
        if !is_right {
            neighbors.push(index + 1);
        }
        if !is_bottom {
            neighbors.push(index + self.line_len);
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

fn find_low_points(input: &Input) -> Vec<(usize, Level)> {
    input
        .vec
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, lvl)| input.get_neighbors(*i).iter().all(|n| n > lvl))
        .collect()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> u32 {
    find_low_points(input).iter().map(|(_, lvl)| lvl + 1).sum()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> u32 {
    let low_points = find_low_points(input);
    let mut basins: Vec<_> = low_points.iter().map(|(pos, _)| (pos, 1)).collect();
    let mut visited: Vec<_> = low_points.iter().map(|lp| lp.0).collect();
    let mut points_to_check: VecDeque<_> = low_points
        .iter()
        .flat_map(|(p, _)| {
            input
                .get_neighbor_positions(*p)
                .into_iter()
                .map(|n| (*p, n))
        })
        .collect();

    dbg!(&low_points, &basins, &visited, &points_to_check);

    while let Some((basin_pos, point)) = points_to_check.pop_front() {
        eprint!(
            "basin: {basin_pos}, point_to_check: {point} - value {} ",
            input.vec[point]
        );
        visited.push(point);
        if input.vec[point] == 9u32 {
            eprintln!("nope");
            continue;
        }
        eprintln!("yep");
        let basin = basins.iter_mut().find(|b| *b.0 == basin_pos).unwrap();
        basin.1 += 1;
        points_to_check.extend(
            input
                .get_neighbor_positions(point)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .map(|n| (*basin.0, n)),
        );
    }

    dbg!(&basins);
    basins.sort_by_key(|b| b.1);
    basins.iter().rev().take(3).map(|b| b.1).product()
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
        assert_eq!(
            part2(&parse(
                "2199943210
                 3987894921
                 9856789892
                 8767896789
                 9899965678"
            )),
            1134
        );
    }
}
