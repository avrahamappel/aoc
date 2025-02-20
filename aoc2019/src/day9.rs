use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<i32>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> String {
    todo!()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intcode_v4() {
        for (program, input, output) in [
            (
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                None,
                vec![
                    109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
                ],
            ),
            (
                "1102,34915192,34915192,7,4,7,99,0",
                None,
                vec![1219070632396864],
            ),
        ] {
            eprintln!("Program: {program}, input: {input:?}");
            assert_eq!(output, run_program(parse(program), input));
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
