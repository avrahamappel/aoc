use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{Intcode, State};

type Input = Vec<i64>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input.split(',').map(|l| l.parse().unwrap()).collect()
}

fn run_program(program: Input, input: Option<i64>) -> Vec<i64> {
    let mut output = vec![];
    let mut prg = Intcode::new(program);
    loop {
        match prg.run(input) {
            State::NeedsInput => unimplemented!(),
            State::Output(o) => output.push(o),
            State::Halted => break,
        }
    }
    output
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> i64 {
    let output = run_program(input.to_owned(), Some(1));
    println!("{output:?}");
    output[0]
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
        for (program, output) in [
            (
                "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
                vec![
                    109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
                ],
            ),
            (
                "1102,34915192,34915192,7,4,7,99,0",
                vec![1_219_070_632_396_864],
            ),
            ("104,1125899906842624,99", vec![1_125_899_906_842_624]),
        ] {
            eprintln!("Program: {program}");
            assert_eq!(output, run_program(parse(program), None));
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
