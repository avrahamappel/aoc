use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse(input: &str) -> String {
    input.to_string()
}

fn is_abba(d: char, prev1: Option<char>, prev2: Option<char>, prev3: Option<char>) -> bool {
    if let Some((a, (b, c))) = prev3.zip(prev2.zip(prev1)) {
        a != b && a == d && b == c
    } else {
        false
    }
}

fn supports_tls(address: &str) -> bool {
    let mut in_brackets = false;
    let mut prev1 = None;
    let mut prev2 = None;
    let mut prev3 = None;
    let mut has_abba = false;
    let mut has_abba_in_hypernet = false;

    for c in address.chars() {
        match c {
            '[' => {
                in_brackets = true;
                prev1 = None;
                prev2 = None;
                prev3 = None;
            }
            ']' => {
                in_brackets = false;
                prev1 = None;
                prev2 = None;
                prev3 = None;
            }
            _ => {
                if is_abba(c, prev1, prev2, prev3) {
                    if in_brackets {
                        has_abba_in_hypernet = true;
                    } else {
                        has_abba = true;
                    }
                }
                prev3 = prev2;
                prev2 = prev1;
                prev1 = Some(c);
            }
        }
    }

    has_abba && !has_abba_in_hypernet
}

#[aoc(day7, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|address: &&str| supports_tls(address))
        .count()
}

fn supports_ssl(address: &str) -> bool {
    let (plain, hypernet) = {
        let mut in_brackets = false;
        let mut plain = vec![];
        let mut hypernet = vec![];

        for c in address.chars() {
            match c {
                '[' => {
                    in_brackets = true;
                }
                ']' => {
                    in_brackets = false;
                }
                _ => {
                    if in_brackets {
                        hypernet.push(c);
                    } else {
                        plain.push(c);
                    }
                }
            }
        }
        (plain, hypernet)
    };

    let has_aba = |cs: &&[_]| cs[0] != cs[1] && cs[0] == cs[2];

    let abas: Vec<_> = plain.windows(3).filter(has_aba).collect();

    hypernet
        .windows(3)
        .filter(has_aba)
        .any(|bab| abas.iter().any(|aba| aba[0] == bab[1] && aba[1] == bab[0]))
}

#[aoc(day7, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|address| supports_ssl(address))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (input, output) in [
            ("abba[mnop]qrst", true),
            ("abcd[bddb]xyyx", false),
            ("aaaa[qwer]tyui", false),
            ("ioxxoj[asdfgh]zxcvbn", true),
        ] {
            eprintln!("testing: [{input}]");
            assert_eq!(output, supports_tls(input));
        }
    }

    #[test]
    fn part2_example() {
        for (input, output) in [
            ("aba[bab]xyz", true),
            ("xyx[xyx]xyx", false),
            ("aaa[kek]eke", true),
            ("zazbz[bzb]cdb", true),
        ] {
            eprintln!("testing: [{input}]");
            assert_eq!(output, supports_ssl(input));
        }
    }
}
