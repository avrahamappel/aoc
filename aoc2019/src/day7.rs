use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::day5;

type Input = day5::Program;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    day5::parse(input)
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> i32 {
    let mut max_output = 0;
    for permutation in (0..=4).permutations(5) {
        dbg!(&permutation);
        for setting in permutation {
            let mut prg = input.to_owned();
            let mut output = day5::run_program(&mut prg, setting);
            if output > max_output {
                max_output = output;
            }
        }
        dbg!(max_output);
    }
    max_output
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (signal,program)in[ ( 43210, "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), ( 54321, "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), ( 65210, "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), ]{
        assert_eq!(part1(&parse(program)), signal);
    }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
