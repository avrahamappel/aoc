use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

type Presents = u32;
type HouseNumber = u32;

#[aoc_generator(day20)]
fn parse(input: &str) -> Presents {
    input.parse().unwrap()
}

fn presents_at_house_number(hn: HouseNumber) -> Presents {
    (1..=Presents::MAX)
        .take_while(|e| hn / e > 0)
        .filter(|e| hn % e == 0)
        .map(|e| e * 10)
        .sum()
}

#[aoc(day20, part1)]
fn part1(input: &Presents) -> HouseNumber {
    let mut counter = 0;

    let mut high = *input;
    let mut low = 1;

    loop {
        counter += 1;
        if counter == *input {
            break 0;
        }

        let hn = ((high - low) / 2) + low;
        eprintln!("{hn}");
        let prs = presents_at_house_number(hn);
        eprintln!("{prs}");
        match prs.cmp(input) {
            Ordering::Equal => break hn,
            Ordering::Greater => high = hn,
            Ordering::Less => low = hn,
        }
    }
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
