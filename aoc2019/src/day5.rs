use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::{Intcode, State};

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|s| s.parse().unwrap()).collect())
}

fn run_program(prg: &mut Intcode, input: i32) -> Option<i32> {
    let mut output = None;
    loop {
        let state = prg.run(Some(input));
        match state {
            State::Halted => return output,
            State::Output(o) => output = Some(o),
            State::NeedsInput => {}
        }
    }
}

#[aoc(day5, part1)]
fn part1(input: &Intcode) -> i32 {
    let mut prg = input.to_owned();
    run_program(&mut prg, 1).unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Intcode) -> i32 {
    let mut prg = input.to_owned();
    run_program(&mut prg, 5).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        for (program, input, output) in [
            ("3,0,4,0,99", 5, Some(5)),
            ("1002,4,3,4,33", 5, None),
            ("1101,100,-1,4,0", 5, None),
        ] {
            eprintln!("Program: {program}, input: {input}");
            let mut prg = parse(program);
            assert_eq!(output, run_program(&mut prg, input));
        }
    }

    #[test]
    fn test_run_program_with_new_opcodes() {
        for (program, input, output) in [
            // Position mode
            // Checks if input is equal to 8
            ("3,9,8,9,10,9,4,9,99,-1,8", 8,1),
            ("3,9,8,9,10,9,4,9,99,-1,8", 7,0),
            // Checks if input less than 8
            ("3,9,7,9,10,9,4,9,99,-1,8", 7,1),
            ("3,9,7,9,10,9,4,9,99,-1,8", 8,0),
            // Immediate mode
            // Checks if input is equal to 8
            ("3,3,1108,-1,8,3,4,3,99", 8,1),
            ("3,3,1108,-1,8,3,4,3,99", 7,0),
            // Checks if input less than 8
            ("3,3,1107,-1,8,3,4,3,99", 8,0),
            ("3,3,1107,-1,8,3,4,3,99", 7,1),
            // Check if input is non-zero, position mode
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 5,1),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0,0),
            // Check if input is non-zero, position mode
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 5,1),
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0,0),
            // Compare with 8, more convoluted
            ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7, 999),
            ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8,1000),
            ("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 9,1001),
        ] {
            eprintln!("Program: {program}, input: {input}");
            let mut prg = parse(program);
            assert_eq!(run_program(&mut prg, input).unwrap(), output);
        }
    }
}
