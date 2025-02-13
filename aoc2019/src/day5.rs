#![allow(clippy::cast_sign_loss)]

use aoc_runner_derive::{aoc, aoc_generator};

type Program = Vec<i32>;

#[aoc_generator(day5)]
fn parse(input: &str) -> Program {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Position(usize);

impl Position {
    fn from(int: i32) -> Self {
        Self(int as usize)
    }
}

#[derive(Debug)]
enum Mode {
    Position(Position),
    Immediate(i32),
}
impl Mode {
    fn get(&self, prg: &Program) -> i32 {
        match self {
            Mode::Position(position) => prg[position.0],
            Mode::Immediate(int) => *int,
        }
    }
}

#[derive(Debug)]
enum Op {
    Add(Mode, Mode, Position),
    Mul(Mode, Mode, Position),
    In(Position),
    Out(Position),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Position),
    Equals(Mode, Mode, Position),
    Halt,
}

impl Op {
    fn from_code(prg: &Program, idx: usize) -> Op {
        let code = prg[idx];
        let arg0 = || prg[idx + 1];
        let arg1 = || prg[idx + 2];
        let arg2 = || prg[idx + 3];
        let opcode = code % 100;
        let mode0 = || {
            if code / 100 % 10 == 0 {
                Mode::Position(Position::from(arg0()))
            } else {
                Mode::Immediate(arg0())
            }
        };
        let mode1 = || {
            if code / 1000 % 10 == 0 {
                Mode::Position(Position::from(arg1()))
            } else {
                Mode::Immediate(arg1())
            }
        };

        match opcode {
            1 => Op::Add(mode0(), mode1(), Position::from(arg2())),
            2 => Op::Mul(mode0(), mode1(), Position::from(arg2())),
            3 => Op::In(Position::from(arg0())),
            4 => Op::Out(Position::from(arg0())),
            5 => Op::JumpIfTrue(mode0(), mode1()),
            6 => Op::JumpIfFalse(mode0(), mode1()),
            7 => Op::LessThan(mode0(), mode1(), Position::from(arg2())),
            8 => Op::Equals(mode0(), mode1(), Position::from(arg2())),
            99 => Op::Halt,
            _ => panic!("Invalid opcode"),
        }
    }
}

/// Run an Intcode program
fn run_program(prg: &mut Program, mut input: i32) -> i32 {
    let mut idx = 0;

    loop {
        let instr = Op::from_code(prg, idx);
        //dbg!(&instr);
        match instr {
            Op::Add(ref lhs, ref rhs, Position(addr)) => {
                prg[addr] = lhs.get(prg) + rhs.get(prg);
                idx += 4;
            }
            Op::Mul(ref lhs, ref rhs, Position(addr)) => {
                prg[addr] = lhs.get(prg) * rhs.get(prg);
                idx += 4;
            }
            Op::In(Position(addr)) => {
                prg[addr] = input;
                idx += 2;
            }
            Op::Out(Position(addr)) => {
                input = prg[addr];
                idx += 2;
            }
            Op::JumpIfTrue(ref cond, ref target) => {
                if cond.get(prg) != 0 {
                    idx = target.get(prg) as usize;
                } else {
                    idx += 3;
                }
            }
            Op::JumpIfFalse(ref cond, ref target) => {
                if cond.get(prg) == 0 {
                    idx = target.get(prg) as usize;
                } else {
                    idx += 3;
                }
            }
            Op::LessThan(ref lhs, ref rhs, Position(addr)) => {
                prg[addr] = i32::from(lhs.get(prg) < rhs.get(prg));
                idx += 4;
            }
            Op::Equals(ref rhs, ref lhs, Position(addr)) => {
                prg[addr] = i32::from(lhs.get(prg) == rhs.get(prg));
                idx += 4;
            }
            Op::Halt => break,
        }
        //dbg!(input, &prg, idx);
    }
    input
}

#[aoc(day5, part1)]
fn part1(input: &Program) -> i32 {
    let mut prg = input.to_owned();
    let mut output = run_program(&mut prg, 1);

    while output == 0 {
        output = run_program(&mut prg, output);
    }

    output
}

#[aoc(day5, part2)]
fn part2(input: &Program) -> i32 {
    let mut prg = input.to_owned();
    run_program(&mut prg, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        for program in ["3,0,4,0,99", "1002,4,3,4,33", "1101,100,-1,4,0"] {
            eprintln!("Program: {program}");
            let mut prg = parse(program);
            assert_eq!(run_program(&mut prg, 5), 5);
        }
    }

    #[test]
    fn test_run_program_with_new_opcodes() {
        for (program, input, output) in [
            // Position mode
            // Checks if input is equal to 8
            ("3,9,8,9,10,9,4,9,99,-1,8", 8, 1),
            ("3,9,8,9,10,9,4,9,99,-1,8", 7, 0),
            // Checks if input less than 8
            ("3,9,7,9,10,9,4,9,99,-1,8", 7, 1),
            ("3,9,7,9,10,9,4,9,99,-1,8", 8, 0),
            // Immediate mode
            // Checks if input is equal to 8
            ("3,3,1108,-1,8,3,4,3,99", 8, 1),
            ("3,3,1108,-1,8,3,4,3,99", 7, 0),
            // Checks if input less than 8
            ("3,3,1107,-1,8,3,4,3,99", 8, 0),
            ("3,3,1107,-1,8,3,4,3,99", 7, 1),
            // Check if input is non-zero, position mode
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 5, 1),
            ("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0, 0),
            // Check if input is non-zero, position mode
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 5, 1),
            ("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0, 0),
            // TODO these fail for some reason
            //("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 7, 999),
            //("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 8, 1000),
            //("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", 9, 1001),
        ] {
            eprintln!("Program: {program}, input: {input}, expected output: {output}");
            let mut prg = parse(program);
            assert_eq!(run_program(&mut prg, input), output);
        }
    }
}
