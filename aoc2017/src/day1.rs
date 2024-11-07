use std::iter::once;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input
        .windows(2)
        .filter(|w| match w.len() {
            2 => w[0] == w[1],
            _ => false,
        })
        .map(|w| w[0])
        // Add the beginning to the end if they match
        .chain(once(
            if input.len() > 1 && input[0] == input[input.len() - 1] {
                input[0]
            } else {
                0
            },
        ))
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (input, expected) in [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)] {
            //eprintln!();
            //eprintln!("---TEST---");
            assert_eq!(part1(&parse(input)), expected);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
