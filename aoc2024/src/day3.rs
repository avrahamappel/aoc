use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> String {
    let mut instrs = vec![];

    for (i, _) in input.match_indices("mul(") {
        let mut digit1 = String::new();
        for (j, c) in input[i..].enumerate() {
            if !c.is_ascii_digit() {
                break;
            }
        }
    }
}

#[aoc(day3, part2)]
fn part2(input: &str) -> String {
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
