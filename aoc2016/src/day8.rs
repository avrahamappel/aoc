use aoc_runner_derive::{aoc, aoc_generator};

enum Command {
    Rect { x: usize, y: usize },
    RotateRow { y: usize, dist: usize },
    RotateCol { x: usize, dist: usize },
}

type Input = Vec<Command>;

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut words = l.trim().split_ascii_whitespace();
            match words.next().unwrap() {
                "rect" => {
                    let dmns = words.next().unwrap();
                    let (x, y) = dmns.split_once('x').unwrap();
                    Command::Rect {
                        x: x.parse().unwrap(),
                        y: y.parse().unwrap(),
                    }
                }
                "rotate" => {
                    match words.next().unwrap() {
                        "row" => {
                            let y = words
                                .next()
                                .unwrap()
                                .trim_start_matches("y=")
                                .parse()
                                .unwrap();
                            let _ = words.next(); // "by"
                            let dist = words.next().unwrap().parse().unwrap();
                            Command::RotateRow { y, dist }
                        }
                        "column" => {
                            let x = words
                                .next()
                                .unwrap()
                                .trim_start_matches("x=")
                                .parse()
                                .unwrap();
                            let _ = words.next(); // "by"
                            let dist = words.next().unwrap().parse().unwrap();
                            Command::RotateCol { x, dist }
                        }
                        _ => {
                            unimplemented!()
                        }
                    }
                }
                _ => {
                    unimplemented!()
                }
            }
        })
        .collect()
}

// 50x6 grid
type Grid = [[bool; 50]; 6];

impl Command {
    fn run(&self, grid: &mut Grid) {
        match self {
            Command::Rect { x, y } => {
                for r in grid.iter_mut().take(*y) {
                    for p in r.iter_mut().take(*x) {
                        *p = true;
                    }
                }
            }
            Command::RotateRow { y, dist } => {
                let n = grid[*y].len();
                let dist = dist.rem_euclid(n); // handle negative and out-of-bounds dist

                // create a temporary vector to store the row
                let row = grid[*y];

                // shift the row elements
                for i in 0..n {
                    grid[*y][i] = row[(i + n - dist) % n];
                }
            }
            Command::RotateCol { x, dist } => {
                let n = grid.len();
                let dist = dist.rem_euclid(n); // Handle negative and out-of-bounds dist

                // Create a temporary vector to store the column
                let col = (0..n).map(|i| grid[i][*x]).collect::<Vec<_>>();

                // Shift the column elements
                for i in 0..n {
                    grid[i][*x] = col[(i + n - dist) % n];
                }
            }
        }
    }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> usize {
    let mut grid = [[false; 50]; 6];
    for cmd in input {
        cmd.run(&mut grid);

        println!();
        for r in grid {
            for p in r {
                print!("{}", if p { '#' } else { '.' });
            }
            println!();
        }
    }
    grid.iter().map(|y| y.iter().filter(|x| **x).count()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "rect 3x2
                 rotate column x=1 by 1
                 rotate row y=0 by 4
                 rotate column x=1 by 1"
            )),
            6
        );
    }
}
