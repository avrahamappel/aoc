use aoc_runner_derive::{aoc, aoc_generator};

type Program = Vec<isize>;

#[aoc_generator(day5)]
fn parse(input: &str) -> Program {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

/// Run an Intcode program
fn run_program(mut prg: Vec<usize>) -> Vec<usize> {
    let mut idx = 0;
    loop {
        let instr = prg[idx];
        match instr {
            1 => {
                let [lhs, rhs, addr] = prg[idx + 1..=idx + 3] else {
                    unimplemented!()
                };
                prg[addr] = prg[lhs] + prg[rhs];
                idx += 4;
            }
            2 => {
                let [lhs, rhs, addr] = prg[idx + 1..=idx + 3] else {
                    unimplemented!()
                };
                prg[addr] = prg[lhs] * prg[rhs];
                idx += 4;
            }
            //Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
            //Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
    prg
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    todo!()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("<EXAMPLE>")), "<RESULT>");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
