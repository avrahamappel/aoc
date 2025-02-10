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
            99 => Op::Halt,
            _ => panic!("Invalid opcode"),
        }
    }

    fn next_idx(self, idx: usize) -> usize {
        idx + match self {
            Op::Add(_, _, _) | Op::Mul(_, _, _) => 4,
            Op::In(_) | Op::Out(_) => 2,
            Op::Halt => unreachable!(),
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
            }
            Op::Mul(ref lhs, ref rhs, Position(addr)) => {
                prg[addr] = lhs.get(prg) * rhs.get(prg);
            }
            Op::In(Position(addr)) => {
                prg[addr] = input;
            }
            Op::Out(Position(addr)) => {
                input = prg[addr];
            }
            Op::Halt => break,
        }
        idx = instr.next_idx(idx);
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
fn part2(input: &Program) -> String {
    todo!()
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
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
