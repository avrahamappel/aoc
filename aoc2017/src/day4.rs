use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let mut words = vec![];
            for word in l.split_ascii_whitespace() {
                if words.contains(&word) {
                    return false;
                }
                words.push(word);
            }
            true
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let mut anagrams = vec![];
            for word in l.split_ascii_whitespace() {
                let mut anagram = vec![];
                anagram.extend(word.chars());
                anagram.sort_unstable();
                //dbg!(&anagram);
                if anagrams.contains(&anagram) {
                    return false;
                }
                anagrams.push(anagram);
                //dbg!(&anagrams);
            }
            true
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
                "aa bb cc dd ee
                 aa bb cc dd aa
                 aa bb cc dd aaa"
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "abcde fghij
                 abcde xyz ecdab
                 a ab abc abd abf abj
                 iiii oiii ooii oooi oooo
                 oiii ioii iioi iiio"
            )),
            3
        );
    }
}
