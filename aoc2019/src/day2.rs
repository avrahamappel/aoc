use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
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
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }
    prg
}

/// Run program with updated values for addresses 1 and 2
fn run_prg_with_nv(noun: usize, verb: usize, mut prg: Vec<usize>) -> usize {
    prg[1] = noun;
    prg[2] = verb;

    prg = run_program(prg);
    prg[0]
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> usize {
    let prg = input.to_vec();

    // Restore 1012 program alarm state
    run_prg_with_nv(12, 2, prg)
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> usize {
    let expected = 19690720;
    let mut noun = 0;
    let mut verb = 0;

    //dbg!(input.len());

    'noun: for n in noun..input.len() {
        'verb: for v in verb..input.len() {
            let prg = input.to_vec();
            let output = run_prg_with_nv(n, v, prg);
            //dbg!((n, v, output));

            if output > expected {
                // Too high
                break 'verb;
            }

            if output == expected {
                // Found the correct inputs, go to end
                noun = n;
                verb = v;
                break 'noun;
            }
        }
    }

    100 * noun + verb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program() {
        for (input, expected) in [
            (
                "1,9,10,3,2,3,11,0,99,30,40,50",
                "3500,9,10,70,2,3,11,0,99,30,40,50",
            ),
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ] {
            let prg = run_program(parse(input));
            let prg = prg
                .iter()
                .map(usize::to_string)
                .collect::<Vec<_>>()
                .join(",");
            assert_eq!(prg, expected);
        }
    }
}
