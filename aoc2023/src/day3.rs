use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

type Position = (usize, usize);

#[derive(Debug)]
enum Component {
    Number(String),
    Symbol(char),
}

struct Schematic {
    grid_len: usize,
    line_len: usize,
    components: HashMap<Position, Component>,
}

impl Schematic {
    fn neighbors(&self, (x, y): Position, str_len: usize) -> HashSet<Position> {
        let mut nbrs = HashSet::new();

        let mut add_nbrs = |x, y| {
            if y != 0 {
                if x != 0 {
                    nbrs.insert((x - 1, y - 1));
                }
                nbrs.insert((x, y - 1));
                if x < self.line_len {
                    nbrs.insert((x + 1, y - 1));
                }
            }
            if x != 0 {
                nbrs.insert((x - 1, y));
            }
            if x < self.line_len {
                nbrs.insert((x + 1, y));
            }
            if y < self.grid_len {
                if x != 0 {
                    nbrs.insert((x - 1, y + 1));
                }
                nbrs.insert((x, y + 1));
                if x < self.line_len {
                    nbrs.insert((x + 1, y + 1));
                }
            }
        };

        for i in 0..str_len {
            add_nbrs(x + i, y);
        }

        nbrs
    }
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Schematic {
    let input = input.trim();
    let grid_len = input.lines().count();
    let line_len = input.chars().position(|c| c == '\n').unwrap();

    // Parse components
    let chars = input
        .lines()
        .flat_map(|l| l.trim().chars())
        .collect::<Vec<_>>();
    let mut c_idx = 0;
    let mut components = HashMap::new();

    while let Some(c) = chars.get(c_idx) {
        let pos = (c_idx % grid_len, c_idx / grid_len);
        if c.is_ascii_digit() {
            let start = c_idx;
            while chars.get(c_idx).is_some_and(|c| c.is_ascii_digit()) {
                c_idx += 1;
            }
            components.insert(pos, Component::Number(chars[start..c_idx].iter().collect()));
        } else if *c != '.' {
            components.insert(pos, Component::Symbol(*c));
            c_idx += 1;
        } else {
            c_idx += 1;
        }
    }

    Schematic {
        grid_len,
        line_len,
        components,
    }
}

#[aoc(day3, part1)]
fn part1(input: &Schematic) -> u32 {
    // for each number in grid
    input
        .components
        .iter()
        .filter_map(|(pos, comp)| match comp {
            Component::Number(n) => Some((n, input.neighbors(*pos, n.len()))),
            Component::Symbol(_) => None,
        })
        // if adjacent cell has symbol
        .filter(|(_, nbrs)| {
            nbrs.iter().any(|pos| {
                input
                    .components
                    .get(pos)
                    .is_some_and(|comp| matches!(comp, Component::Symbol(_)))
            })
        })
        // add all together
        .map(|(n, _)| n.parse::<u32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Schematic) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "467..114..
                 ...*......
                 ..35..633.
                 ......#...
                 617*......
                 .....+.58.
                 ..592.....
                 ......755.
                 ...$.*....
                 .664.598.."
            )),
            4361
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
