use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<i32>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> String {
    todo!()
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
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
