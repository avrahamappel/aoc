use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for (_, [d1, d2]) in re.captures_iter(input).map(|c| c.extract()) {
        println!("mul({d1},{d2})");
        let d1: u32 = d1.parse().unwrap();
        let d2: u32 = d2.parse().unwrap();
        sum += d1 * d2;
    }

    sum
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut doing = true;

    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    for matches in re.captures_iter(input) {
        match matches.get(0).unwrap().as_str() {
            "do()" => {
                doing = true;
            }
            "don't()" => {
                doing = false;
            }
            _ => {
                if doing {
                    let d1: u32 = matches.get(1).unwrap().as_str().parse().unwrap();
                    let d2: u32 = matches.get(2).unwrap().as_str().parse().unwrap();
                    println!("mul({d1},{d2})");
                    sum += d1 * d2;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
