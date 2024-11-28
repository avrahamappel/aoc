use std::cmp::{max, min};

use aoc_runner_derive::{aoc, aoc_generator};

type Presents = u32;
type HouseNumber = u32;

#[aoc_generator(day20)]
fn parse(input: &str) -> Presents {
    input.parse().unwrap()
}

fn integer_sqrt(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut left = 1;
    let mut right = n;

    while left < right {
        let mid = (left + right + 1) / 2; // Use upper mid to avoid infinite loop
        if mid * mid <= n {
            left = mid; // mid is a candidate
        } else {
            right = mid - 1; // mid is too large
        }
    }

    left
}

fn presents_at_house_number(hn: HouseNumber) -> Presents {
    let mut prs = 0;
    for p in 1..=integer_sqrt(hn) {
        if hn % p == 0 {
            prs += p;
            if p != hn / p {
                prs += hn / p;
            }
        }
    }
    prs
}

fn next_prime(n: u32) -> u32 {
    let mut next = n + 1;
    loop {
        if is_prime(next) {
            return next;
        }
        next += 1;
    }
}

fn prev_prime(n: u32) -> Option<u32> {
    let mut prev = n - 1;
    while prev > 1 {
        if is_prime(prev) {
            return Some(prev);
        }
        prev -= 1;
    }
    None
}

fn is_prime(n: u32) -> bool {
    if n == 0 {
        return false;
    }
    if n == 1 {
        return true;
    }

    for i in 2..=integer_sqrt(n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find_lowest_n(k: u32) -> Option<u32> {
    let mut lower_bound = 1;
    let mut upper_bound = k;
    let mut best_n = None;

    while lower_bound <= upper_bound {
        let mid = (lower_bound + upper_bound) / 2;
        let sigma_mid = presents_at_house_number(mid);

        if sigma_mid < k {
            // Increase n
            lower_bound = mid + 1;
            // Jump to the next prime
            let next_p = next_prime(mid);
            lower_bound = max(lower_bound, next_p);
        } else {
            // Found a candidate
            best_n = Some(mid);
            // Try to find a smaller n
            upper_bound = mid - 1;
            // Jump to the previous prime
            if let Some(prev_p) = prev_prime(mid) {
                upper_bound = min(upper_bound, prev_p);
            }
        }
    }

    best_n
}

#[aoc(day20, part1)]
fn part1(input: &Presents) -> HouseNumber {
    find_lowest_n(*input).unwrap_or(HouseNumber::MAX)
}

#[aoc(day20, part2)]
fn part2(input: &Presents) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (hn, prs) in [(1, 10), (4, 70), (9, 130)] {
            assert_eq!(hn, part1(&prs));
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
