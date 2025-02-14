#![allow(clippy::cast_sign_loss)]

use std::ops::Index;

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
    fn get(&self, prg: &Intcode) -> i32 {
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
    fn from_code(prg: &Intcode, idx: usize) -> Op {
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

#[derive(Clone)]
pub struct Intcode {
    program: Vec<i32>,
}

impl Index<usize> for Intcode {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.program[index]
    }
}

impl Intcode {
    pub fn new(program: Vec<i32>) -> Self {
        Self { program }
    }

    /// Run an Intcode program
    pub fn run(&mut self, input: &[i32]) -> Vec<i32> {
        let mut idx = 0;
        let mut input_iter = input.iter();
        let mut output = vec![];

        loop {
            let instr = Op::from_code(self, idx);
            //dbg!(&instr);
            match instr {
                Op::Add(ref lhs, ref rhs, Position(addr)) => {
                    self.program[addr] = lhs.get(self) + rhs.get(self);
                    idx += 4;
                }
                Op::Mul(ref lhs, ref rhs, Position(addr)) => {
                    self.program[addr] = lhs.get(self) * rhs.get(self);
                    idx += 4;
                }
                Op::In(Position(addr)) => {
                    self.program[addr] = *input_iter
                        .next()
                        .expect("input did not contain enough values");
                    idx += 2;
                }
                Op::Out(Position(addr)) => {
                    output.push(self.program[addr]);
                    idx += 2;
                }
                Op::JumpIfTrue(ref cond, ref target) => {
                    if cond.get(self) != 0 {
                        idx = target.get(self) as usize;
                    } else {
                        idx += 3;
                    }
                }
                Op::JumpIfFalse(ref cond, ref target) => {
                    if cond.get(self) == 0 {
                        idx = target.get(self) as usize;
                    } else {
                        idx += 3;
                    }
                }
                Op::LessThan(ref lhs, ref rhs, Position(addr)) => {
                    self.program[addr] = i32::from(lhs.get(self) < rhs.get(self));
                    idx += 4;
                }
                Op::Equals(ref rhs, ref lhs, Position(addr)) => {
                    self.program[addr] = i32::from(lhs.get(self) == rhs.get(self));
                    idx += 4;
                }
                Op::Halt => break,
            }
            //dbg!(input, &self.program, idx);
        }
        output
    }
}
