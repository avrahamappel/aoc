use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Dir {
    U,
    D,
    R,
    L,
}

type Digit = (usize, usize);
type Dirs = Vec<Dir>;
type Lines = Vec<Dirs>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Lines {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .filter_map(|c| match c {
                    'U' => Some(Dir::U),
                    'D' => Some(Dir::D),
                    'R' => Some(Dir::R),
                    'L' => Some(Dir::L),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Lines) -> String {
    let digits = input.iter().fold(vec![], |mut digits: Vec<Digit>, line| {
        let new_digit = line.iter().fold(
            digits.last().copied().unwrap_or((1, 1)),
            |(x, y), dir| match dir {
                Dir::U => (x, if y > 0 { y - 1 } else { 0 }),
                Dir::D => (x, if y < 2 { y + 1 } else { 2 }),
                Dir::L => (if x > 0 { x - 1 } else { 0 }, y),
                Dir::R => (if x < 2 { x + 1 } else { 2 }, y),
            },
        );
        digits.push(new_digit);
        digits
    });

    digits
        .into_iter()
        .map(|(x, y)| ((x + 1) + (y * 3)).to_string())
        .collect()
}

#[aoc(day2, part2)]
fn part2(input: &Lines) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "ULL
                 RRDDD
                 LURDL
                 UUUUD"
            )),
            "1985"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
