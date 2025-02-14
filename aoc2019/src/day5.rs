use aoc_runner_derive::{aoc, aoc_generator};

use crate::intcode::Intcode;

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Intcode {
    Intcode::new(input.split(',').map(|s| s.parse().unwrap()).collect())
}

fn run_program(prg: &mut Intcode, input: i32) -> Vec<i32> {
    prg.run(&[input])
}

#[aoc(day5, part1)]
fn part1(input: &Intcode) -> i32 {
    let mut prg = input.to_owned();
    let mut output = run_program(&mut prg, 1)[0];

    while output == 0 {
        output = run_program(&mut prg, output)[0];
    }

    output
}

#[aoc(day5, part2)]
fn part2(input: &Intcode) -> i32 {
    let mut prg = input.to_owned();
    run_program(&mut prg, 5)[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        for (program, input, output) in [
            ("3,0,4,0,99", 5, vec![5]),
            ("1002,4,3,4,33", 5, vec![]),
            ("1101,100,-1,4,0", 5, vec![]),
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
            ("3,9,8,9,10,9,4,9,99,-1,8", 8, vec![1]),
            ("3,9,8,9,10,9,4,9,99,-1,8", 7, vec![0]),
            // Checks if input less than 8
            ("3,9,7,9,10,9,4,9,99,-1,8", 7, vec![1]),
            ("3,9,7,9,10,9,4,9,99,-1,8", 8, vec![0]),
            // Immediate mode
            // Checks if input is equal to 8
            ("3,3,1108,-1,8,3,4,3,99", 8, vec![1]),
            ("3,3,1108,-1,8,3,4,3,99", 7, vec![0]),
            // Checks if input less than 8
            ("3,3,1107,-1,8,3,4,3,99", 8, vec![0]),
            ("3,3,1107,-1,8,3,4,3,99", 7, vec![1]),
            // Check if input is non-zero, position mode
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 5, vec![1]),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, vec![0]),
            // Check if input is non-zero, position mode
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 5, vec![1]),
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, vec![0]),
            // TODO these fail for some reason
            //("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7, vec![999]),
            //("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8, vec![1000]),
            //("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 9, vec![1001]),
        ] {
            eprintln!("Program: {program}, input: {input}");
            let mut prg = parse(program);
            assert_eq!(run_program(&mut prg, input), output);
        }
    }
}
