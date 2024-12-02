use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<i32>, Vec<i32>);

#[aoc_generator(day1)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.trim().split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> u32 {
    //eprintln!("{input:?}");
    let mut r#as = input.0.clone();
    let mut bs = input.1.clone();
    r#as.sort_unstable();
    bs.sort_unstable();
    //eprintln!("{as:?}");
    //eprintln!("{bs:?}");
    r#as.iter()
        .zip(bs.iter())
        //.inspect(|pair| {
        //    dbg!(pair);
        //})
        .map(|(a, b)| a.abs_diff(*b))
        //.inspect(|diff| {
        //    dbg!(diff);
        //})
        .sum()
}

#[aoc(day1, part2)]
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
                "3   4
                 4   3
                 2   5
                 1   3
                 3   9
                 3   3"
            )),
            11
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
