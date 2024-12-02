use aoc_runner_derive::{aoc, aoc_generator};

type Level = u32;
type Report = Vec<Level>;
type Input = Vec<Report>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|c| c.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .filter(|report| {
            let mut incr = None;
            let mut prev = None;
            for lvl in *report {
                //eprintln!("{prev:?} {lvl} {incr:?}");
                if let Some(prv) = prev {
                    match incr {
                        None => {
                            incr = Some(lvl > prv);
                        }
                        Some(true) => {
                            if lvl <= prv || lvl - prv > 3 {
                                return false;
                            }
                        }
                        Some(false) => {
                            if lvl >= prv || prv - lvl > 3 {
                                return false;
                            }
                        }
                    }
                }
                prev = Some(lvl);
            }
            true
        })
        //.inspect(|_| {
        //    eprintln!("ok");
        //})
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "7 6 4 2 1
                 1 2 7 8 9
                 9 7 6 2 1
                 1 3 2 4 5
                 8 6 4 4 1
                 1 3 6 7 9"
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
