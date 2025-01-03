//use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Copy)]
struct Light {
    on: bool,
}

#[derive(Clone)]
struct Grid {
    steps: usize,
    line_len: usize,
    vec: Vec<Light>,
}

//impl Display for Grid {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        for line in self.vec.chunks(self.line_len) {
//            for light in line {
//                if light.on {
//                    write!(f, "#")?;
//                } else {
//                    write!(f, ".")?;
//                }
//            }
//            writeln!(f)?;
//        }
//        Ok(())
//    }
//}

#[aoc_generator(day18)]
fn parse(input: &str) -> Grid {
    let mut lines = input.lines().peekable();
    let line_len = lines.peek().unwrap().len();
    let vec = lines
        .flat_map(|line| {
            line.trim().chars().map(|c| {
                let on = c == '#';
                Light { on }
            })
        })
        .collect();
    Grid {
        line_len,
        vec,
        steps: 100,
    }
}

impl Grid {
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

    /// Get neighbors of the given light index
    fn get_neighbors(&self, index: usize) -> Vec<Light> {
        assert!(index <= self.vec.len());

        // Check if index is along the borders of the grid
        let is_top = index <= self.top_right_corner();
        let is_bottom = index >= self.bottom_left_corner();
        let is_left = index % self.line_len == 0;
        let is_right = (index % self.line_len) == (self.line_len - 1);

        // Add neighbors to vec
        let mut neighbors = Vec::with_capacity(8);
        if !is_top {
            if !is_left {
                neighbors.push(self.vec[index - self.line_len - 1]);
            }
            neighbors.push(self.vec[index - self.line_len]);
            if !is_right {
                neighbors.push(self.vec[index - self.line_len + 1]);
            }
        }
        if !is_left {
            neighbors.push(self.vec[index - 1]);
        }
        if !is_right {
            neighbors.push(self.vec[index + 1]);
        }
        if !is_bottom {
            if !is_left {
                neighbors.push(self.vec[index + self.line_len - 1]);
            }
            neighbors.push(self.vec[index + self.line_len]);
            if !is_right {
                neighbors.push(self.vec[index + self.line_len + 1]);
            }
        }

        neighbors
    }

    fn step(&mut self) {
        let mut new_grid = Vec::with_capacity(self.vec.len());
        for (i, light) in self.vec.iter().enumerate() {
            // get count of lit neighbors
            let lit_neighbors = self.get_neighbors(i).iter().filter(|l| l.on).count();
            let on = if light.on {
                // stay on if count is 2 or 3
                lit_neighbors == 2 || lit_neighbors == 3
            } else {
                // turn on if count == 3
                lit_neighbors == 3
            };
            // push new value into new vec
            new_grid.push(Light { on });
        }

        // set new vec to self.vec
        self.vec = new_grid;
    }

    /// Step 2, corners always stay on
    fn step_with_fixed_corners(&mut self) {
        let mut new_grid = Vec::with_capacity(self.vec.len());

        let fixed_corners = [
            self.top_left_corner(),
            self.top_right_corner(),
            self.bottom_left_corner(),
            self.bottom_right_corner(),
        ];

        for (i, light) in self.vec.iter().enumerate() {
            // get count of lit neighbors
            let lit_neighbors = self.get_neighbors(i).iter().filter(|l| l.on).count();
            let on = if fixed_corners.contains(&i) {
                true
            } else if light.on {
                // stay on if count is 2 or 3
                lit_neighbors == 2 || lit_neighbors == 3
            } else {
                // turn on if count == 3
                lit_neighbors == 3
            };
            // push new value into new vec
            new_grid.push(Light { on });
        }

        // set new vec to self.vec
        self.vec = new_grid;
    }

    fn num_lit(&self) -> usize {
        self.vec.iter().filter(|l| l.on).count()
    }
}

#[aoc(day18, part1)]
fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for _ in 0..grid.steps {
        grid.step();
    }
    grid.num_lit()
}

#[aoc(day18, part2)]
fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    for _ in 0..grid.steps {
        //eprintln!("{grid}");
        grid.step_with_fixed_corners();
    }
    grid.num_lit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let mut grid = parse(
            ".#.#.#
            ...##.
            #....#
            ..#...
            #.#..#
            ####..",
        );
        grid.steps = 4;

        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn part2_example() {
        let mut grid = parse(
            "##.#.#
            ...##.
            #....#
            ..#...
            #.#..#
            ####.#",
        );
        grid.steps = 5;

        assert_eq!(part2(&grid), 17);
    }
}
