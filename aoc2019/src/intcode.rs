#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::must_use_candidate)]

use std::{
    fmt::{Display, Formatter},
    ops::{Index, IndexMut},
};

#[derive(Debug)]
enum Mode {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Mode::Position(addr) => write!(f, "${addr}")?,
            Mode::Immediate(int) => write!(f, "{int}")?,
            Mode::Relative(int) => {
                if int.is_positive() {
                    write!(f, "${{CRB + {int}}}")?;
                } else {
                    write!(f, "${{CRB - {}}}", int.abs())?;
                }
            }
        }
        Ok(())
    }
}

impl Mode {
    fn new(code: i64, arg: i64) -> Self {
        match code {
            0 => Mode::Position(arg),
            1 => Mode::Immediate(arg),
            2 => Mode::Relative(arg),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
enum Op {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    In(Mode),
    Out(Mode),
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    AdjRelativeBase(Mode),
    Halt,
}

impl Op {
    fn from_code(prg: &Intcode, idx: usize) -> Op {
        let code = prg[idx];
        let arg0 = || prg[idx + 1];
        let arg1 = || prg[idx + 2];
        let arg2 = || prg[idx + 3];
        let opcode = code % 100;
        let mode0 = || Mode::new(code / 100 % 10, arg0());
        let mode1 = || Mode::new(code / 1000 % 10, arg1());
        let mode2 = || Mode::new(code / 10000 % 10, arg2());

        match opcode {
            1 => Op::Add(mode0(), mode1(), mode2()),
            2 => Op::Mul(mode0(), mode1(), mode2()),
            3 => Op::In(mode0()),
            4 => Op::Out(mode0()),
            5 => Op::JumpIfTrue(mode0(), mode1()),
            6 => Op::JumpIfFalse(mode0(), mode1()),
            7 => Op::LessThan(mode0(), mode1(), mode2()),
            8 => Op::Equals(mode0(), mode1(), mode2()),
            9 => Op::AdjRelativeBase(mode0()),
            99 => Op::Halt,
            _ => {
                //dbg!(opcode);
                panic!("Invalid opcode")
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum State {
    NeedsInput,
    Output(i64),
    Halted,
}

#[derive(Clone)]
pub struct Intcode {
    program: Vec<i64>,
    idx: usize,
    relative_base: i64,
}

impl Index<Mode> for Intcode {
    type Output = i64;

    fn index(&self, index: Mode) -> &i64 {
        match index {
            Mode::Position(addr) => &self[addr as usize],
            Mode::Immediate(_) => unimplemented!(),
            Mode::Relative(int) => &self[(int + self.relative_base) as usize],
        }
    }
}

impl IndexMut<Mode> for Intcode {
    fn index_mut(&mut self, index: Mode) -> &mut i64 {
        match index {
            Mode::Position(addr) => &mut self[addr as usize],
            Mode::Immediate(_) => unimplemented!(),
            Mode::Relative(int) => {
                let rb = self.relative_base;
                &mut self[(int + rb) as usize]
            }
        }
    }
}

impl Index<usize> for Intcode {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        if self.program.len() <= index {
            &0
        } else {
            &self.program[index]
        }
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut i64 {
        if self.program.len() <= index {
            self.program.resize(index + 1, 0);
        }

        &mut self.program[index]
    }
}

impl Intcode {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            program,
            idx: 0,
            relative_base: 0,
        }
    }

    // Get a value from a mode
    fn get(&self, mode: Mode) -> i64 {
        if let Mode::Immediate(int) = mode {
            int
        } else {
            self[mode]
        }
    }

    /// Run an Intcode program
    pub fn run(&mut self, mut input: Option<i64>) -> State {
        loop {
            let instr = Op::from_code(self, self.idx);
            //dbg!(&instr);
            match instr {
                Op::Add(lhs, rhs, addr) => {
                    self[addr] = self.get(lhs) + self.get(rhs);
                    self.idx += 4;
                }
                Op::Mul(lhs, rhs, addr) => {
                    self[addr] = self.get(lhs) * self.get(rhs);
                    self.idx += 4;
                }
                Op::In(addr) => {
                    if let Some(i) = input.take() {
                        self[addr] = i;
                        self.idx += 2;
                    } else {
                        return State::NeedsInput;
                    }
                }
                Op::Out(addr) => {
                    let output = self.get(addr);
                    self.idx += 2;
                    return State::Output(output);
                }
                Op::JumpIfTrue(cond, target) => {
                    if self.get(cond) != 0 {
                        self.idx = self.get(target) as usize;
                    } else {
                        self.idx += 3;
                    }
                }
                Op::JumpIfFalse(cond, target) => {
                    if self.get(cond) == 0 {
                        self.idx = self.get(target) as usize;
                    } else {
                        self.idx += 3;
                    }
                }
                Op::LessThan(lhs, rhs, addr) => {
                    self[addr] = i64::from(self.get(lhs) < self.get(rhs));
                    self.idx += 4;
                }
                Op::Equals(rhs, lhs, addr) => {
                    self[addr] = i64::from(self.get(lhs) == self.get(rhs));
                    self.idx += 4;
                }
                Op::AdjRelativeBase(arg) => {
                    self.relative_base += self.get(arg);
                    self.idx += 2;
                }
                Op::Halt => return State::Halted,
            }
            //dbg!(input, &self.program, idx);
        }
    }

    pub fn disassemble(&self) -> String {
        let mut idx = 0;
        let mut lines = vec![];
        let mut parsing_arguments = 0;

        let longest_idx = self.program.len().to_string().len();
        let longest_opc = self.program.iter().max().unwrap_or(&0).to_string().len();

        while idx < self.program.len() {
            //dbg!(&lines);
            let mut display = format!(
                "{idx:ln1$}:  {code:ln2$}  ",
                ln1 = longest_idx,
                ln2 = longest_opc,
                code = self.program[idx]
            );

            if parsing_arguments > 0 {
                parsing_arguments -= 1;
            } else if self.program[idx] == 0 {
                display.push_str("mem");
            } else {
                let op = Op::from_code(self, idx);
                match op {
                    Op::Add(lhs, rhs, addr) => {
                        display.push_str(&format!("{addr} = {lhs} + {rhs}"));
                        parsing_arguments = 3;
                    }
                    Op::Mul(lhs, rhs, addr) => {
                        display.push_str(&format!("{addr} = {lhs} * {rhs}"));
                        parsing_arguments = 3;
                    }
                    Op::In(arg) => {
                        display.push_str(&format!("{arg} = stdin"));
                        parsing_arguments = 1;
                    }
                    Op::Out(arg) => {
                        display.push_str(&format!("print {arg}"));
                        parsing_arguments = 1;
                    }
                    Op::JumpIfTrue(arg, target) => {
                        display.push_str(&format!("if {arg} goto {target}"));
                        parsing_arguments = 2;
                    }
                    Op::JumpIfFalse(arg, target) => {
                        display.push_str(&format!("if !{arg} goto {target}"));
                        parsing_arguments = 2;
                    }
                    Op::LessThan(lhs, rhs, addr) => {
                        display.push_str(&format!("{addr} = {lhs} < {rhs}"));
                        parsing_arguments = 3;
                    }
                    Op::Equals(lhs, rhs, addr) => {
                        display.push_str(&format!("{addr} = {lhs} == {rhs}"));
                        parsing_arguments = 3;
                    }
                    Op::AdjRelativeBase(arg) => {
                        display.push_str(&format!("CRB += {arg}"));
                        parsing_arguments = 1;
                    }
                    Op::Halt => {
                        display.push_str("exit");
                    }
                }
            }
            lines.push(display);
            idx += 1;
        }

        lines.join("\n")
    }
}
