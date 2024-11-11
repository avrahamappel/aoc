use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.trim().to_string()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            let first_dig = line.chars().find(char::is_ascii_digit);
            let last_dig = line.chars().rev().find(char::is_ascii_digit);
            if let (Some(f), Some(l)) = (first_dig, last_dig) {
                Some(format!("{f}{l}"))
            } else {
                None
            }
        })
        .map(|digits| digits.parse::<u32>().unwrap())
        .sum()
}

/// Parse all digits or numeric digit strings from a string
/// Returns a Vec of chars that are valid ascii digits
fn parse_digits(s: &str) -> Vec<char> {
    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    // Found digits in here
    let mut digits = vec![];
    // Track current alphabetical word
    let mut cur_word = String::new();

    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            cur_word.clear();
        } else {
            cur_word.push(c)
        }

        // Check cur_word for digit match
        if let Some(pos) = digit_words.iter().position(|dw| cur_word.ends_with(dw)) {
            // Increment index by 1 to find the char value
            digits.push(char::from_digit(pos as u32 + 1, 10).unwrap());
            cur_word.clear();
        }
    }

    digits
}

// TODO Doesn't work. Perhaps we need to use parse_digits backwards for the second number, just as
// we do for the first?
#[aoc(day1, part2)]
fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .filter_map(|line| {
            eprintln!();
            eprintln!("---LINE---");
            dbg!(line);
            let digits = parse_digits(line);
            dbg!(&digits);
            if let (Some(f), Some(l)) = (digits.first(), digits.last()) {
                Some(format!("{f}{l}"))
            } else {
                None
            }
        })
        .map(|d| d.parse::<u32>().unwrap())
        .inspect(|x| {
            dbg!(x);
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
                "1abc2
                 pqr3stu8vwx
                 a1b2c3d4e5f
                 treb7uchet"
            )),
            142
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "two1nine
                 eightwothree
                 abcone2threexyz
                 xtwone3four
                 4nineeightseven2
                 zoneight234
                 7pqrstsixteen"
            )),
            281
        );
    }
}
