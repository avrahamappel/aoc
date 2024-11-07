use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(str::trim).map(String::from).collect()
}

fn letter_counts(input: &str) -> Vec<(char, usize)> {
    let mut cts: Vec<(_, _)> = vec![];
    for char in input.chars() {
        if let Some((_, count)) = cts.iter_mut().find(|ct| ct.0 == char) {
            *count += 1;
        } else {
            cts.push((char, 1));
        }
    }
    cts
}

#[aoc(day2, part1)]
fn part1(ids: &[String]) -> u32 {
    let mut twice_ct = 0;
    let mut thrice_ct = 0;
    for id in ids {
        let cts = letter_counts(id);
        if cts.iter().any(|ct| ct.1 == 2) {
            twice_ct += 1;
        }
        if cts.iter().any(|ct| ct.1 == 3) {
            thrice_ct += 1;
        }
    }

    twice_ct * thrice_ct
}

#[aoc(day2, part2)]
fn part2(input: &[String]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "abcdef
                 bababc
                 abbcde
                 abcccd
                 aabcdd
                 abcdee
                 ababab"
            )),
            12
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
