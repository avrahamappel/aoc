use std::collections::HashMap;

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

fn check_password2(password: u32) -> bool {
    let digits = password.to_string();
    if digits.len() != 6 {
        return false;
    }

    let mut prev_m = None;
    let mut repeating = HashMap::new();
    for digit in digits.chars() {
        if let Some(prev) = prev_m {
            if prev > digit {
                return false;
            }
            if prev == digit {
                repeating.entry(digit).and_modify(|c| *c += 1).or_insert(2);
            }
        }
        prev_m = Some(digit);
    }

    repeating.values().any(|c| *c == 2)
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    (input.0..input.1).filter(|p| check_password2(*p)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_password() {
        for (input, output) in [(111111, true), (223450, false), (123789, false)] {
            eprintln!("input: {input} output: {output}");
            assert_eq!(output, check_password(input));
        }
    }

    #[test]
    fn test_check_password2() {
        for (input, output) in [(112233, true), (123444, false), (111122, true)] {
            eprintln!("input: {input} output: {output}");
            assert_eq!(output, check_password2(input));
        }
    }
}
