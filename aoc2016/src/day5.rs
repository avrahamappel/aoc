use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut i = 0;

    while password.len() < 8 {
        let val = format!("{input}{i}");
        eprintln!("string:    {val}");
        let hash = md5::compute(val).0;
        eprintln!("hash 0..6: {}", String::from_utf8_lossy(&hash[0..6]));
        if hash.iter().take(5).all(|b| *b == b'0') {
            password.push(char::from(hash[5]));
        }
        i += 1;
    }

    password
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("abc")), "18f47a30");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
