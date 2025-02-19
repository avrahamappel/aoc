use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let area = 25 * 6;
    let layer = input
        .as_bytes()
        .chunks(area)
        .min_by_key(|layer| layer.iter().filter(|c| **c == b'0').count())
        .unwrap();
    let ones = layer.iter().filter(|c| **c == b'1').count();
    let twos = layer.iter().filter(|c| **c == b'2').count();
    ones * twos
}

#[aoc(day8, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn part1_example() {
    //    assert_eq!(part1(&parse("123456789012")), "<RESULT>");
    //}

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
