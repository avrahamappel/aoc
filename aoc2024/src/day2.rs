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

fn check_report(report: &Report, i: usize) -> bool {
    let mut incr = None;
    let mut prev = None;
    for lvl in report {
        //eprintln!("{prev:?} {lvl} {incr:?}");
        if let Some(prv) = prev {
            if incr.is_none() {
                incr = Some(lvl > prv);
            }
            match incr {
                Some(true) => {
                    if lvl <= prv || lvl - prv > 3 {
                        eprintln!(
                            "{}: Increase error: {lvl} does not increase correctly from {prv}",
                            i + 1
                        );
                        return false;
                    }
                }
                Some(false) => {
                    if lvl >= prv || prv - lvl > 3 {
                        eprintln!(
                            "{}: Decrease error: {lvl} does not decrease correctly from {prv}",
                            i + 1
                        );
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        prev = Some(lvl);
    }
    eprintln!("{}: Safe", i + 1);
    true
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, report)| check_report(report, *i))
        //.inspect(|_| {
        //    eprintln!("ok");
        //})
        .count()
}

#[aoc(day2, part2)]
fn part2(input: &Input) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(i, report)| {
            if check_report(*report, *i) {
                return true;
            }

            // try removing each level
            // could make this smarter and only remove ones that are near the area of error, but
            // this is probably fine for now
            for j in 0..report.len() {
                let mut r = (*report).clone();
                r.remove(j);
                if check_report(&r, *i) {
                    return true;
                }
            }

            false
        })
        .count()
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
        assert_eq!(
            part2(&parse(
                "7 6 4 2 1
                 1 2 7 8 9
                 9 7 6 2 1
                 1 3 2 4 5
                 8 6 4 4 1
                 1 3 6 7 9"
            )),
            4
        );
    }
}
