use aoc_runner_derive::{aoc, aoc_generator};

type Input = (u32, u32);

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    let (start, finish) = input.split_once('-').unwrap();
    (start.parse().unwrap(), finish.parse().unwrap())
}

fn check_password(password: u32) -> bool {
    let digits = password.to_string();
    if digits.len() != 6 {
        return false;
    }

    let mut prev_m = None;
    let mut has_repeating = false;
    for digit in digits.chars() {
        if let Some(prev) = prev_m {
            if prev > digit {
                return false;
            }
            if prev == digit {
                has_repeating = true;
            }
        }
        prev_m = Some(digit);
    }

    has_repeating
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    (input.0..input.1).filter(|p| check_password(*p)).count()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password() {
        for (input, output) in [(111111, true), (223450, false), (123789, false)] {
            assert_eq!(output, check_password(input));
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
