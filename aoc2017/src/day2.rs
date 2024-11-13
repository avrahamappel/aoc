use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u32>>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(str::parse::<u32>)
                .filter_map(Result::ok)
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .map(|row| {
            let max = row.iter().max();
            let min = row.iter().min();

            if let Some((m, n)) = max.zip(min) {
                m - n
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> u32 {
    input
        .iter()
        .map(|row| {
            let mut res = 0;
            for n1 in row {
                for n2 in row {
                    if n1 == n2 {
                        // Is is possible for the same value to appear more than once?
                        continue;
                    }

                    if *n2 == 0 {
                        continue;
                    }

                    if n1 % n2 == 0 {
                        res = n1 / n2;
                        break;
                    }
                }
            }
            res
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "5 1 9 5
                 7 5 3
                 2 4 6 8"
            )),
            18
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "5 9 2 8
                 9 4 7 3
                 3 8 6 5"
            )),
            9
        );
    }
}
