use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let first_dig = line.chars().find(char::is_ascii_digit);
            let last_dig = line.chars().rev().find(char::is_ascii_digit);
            if let (Some(f), Some(l)) = (first_dig, last_dig) {
                Some(format!("{f}{l}"))
            } else {
                None
            }
        })
        .map(|digits| digits.parse::<u32>().unwrap())
        .sum()
}

#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "1abc2
                 pqr3stu8vwx
                 a1b2c3d4e5f
                 treb7uchet"
            )),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "two1nine
                 eightwothree
                 abcone2threexyz
                 xtwone3four
                 4nineeightseven2
                 zoneight234
                 7pqrstsixteen"
            )),
            281
        );
    }
}
