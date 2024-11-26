use aoc_runner_derive::{aoc, aoc_generator};

type Spec = (u32, u32, u32);
type Input = Vec<Spec>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut specs = l.split_whitespace();
            let h = specs.next().unwrap().parse().unwrap();
            let w = specs.next().unwrap().parse().unwrap();
            let l = specs.next().unwrap().parse().unwrap();
            (h, w, l)
        })
        .collect()
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|(h, w, l)| h + w > *l && h + l > *w && w + l > *h)
        .count()
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("5 10 25")), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
