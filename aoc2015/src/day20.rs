use aoc_runner_derive::{aoc, aoc_generator};

type Presents = u32;
type HouseNumber = u32;

#[aoc_generator(day20)]
fn parse(input: &str) -> Presents {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
fn part1(input: &Presents) -> HouseNumber {
    // house numbers
    (1..)
        .map(|hn| {
            // elves bearing presents
            let prs: Presents = (1..=hn).filter(|e| hn % e == 0).map(|e| e * 10).sum();
            (hn, prs)
        })
        .inspect(|(hn, prs)| {
            eprintln!("House: {hn}, presents: {prs}");
        })
        .skip_while(|(_, prs)| prs < input)
        .map(|(hn, _)| hn)
        .next()
        .unwrap()
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
        for (hn, prs) in [(1, 10), (4, 70)] {
            assert_eq!(hn, part1(&prs));
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
