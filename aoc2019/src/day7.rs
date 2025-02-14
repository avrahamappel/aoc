use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::intcode::Intcode;

type Input = Vec<i32>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> i32 {
    let mut max_output = 0;
    for permutation in (0..=4).permutations(5) {
        let mut prev_output = 0;
        for setting in permutation {
            //dbg!(setting, max_output, prev_output);
            let mut prg = Intcode::new(input.clone());
            // first setting then previous output or 0
            let inp = [setting, prev_output];
            let output = prg.run(&inp)[0];
            if output > max_output {
                max_output = output;
            }
            prev_output = output;
        }
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
        for (signal,program) in [
            (43210, "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            (54321, "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            (65210, "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
        ] {
            eprintln!("Program: {program}, signal: {signal}");
            assert_eq!(part1(&parse(program)), signal);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
