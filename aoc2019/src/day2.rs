use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(str::parse)
        .filter_map(Result::ok)
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[usize]) -> String {
    let mut prg = input.to_vec();
    let mut idx = 0;
    loop {
        let instr = prg[idx];
        match instr {
            1 => {
                let [lhs, rhs, addr] = prg[idx + 1..idx + 3] else {
                    unimplemented!()
                };
                prg[addr] = lhs + rhs;
                idx += 4;
            }
            2 => {
                let [lhs, rhs, addr] = prg[idx + 1..idx + 3] else {
                    unimplemented!()
                };
                prg[addr] = lhs * rhs;
                idx += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode"),
        }
    }

    prg.iter()
        .map(usize::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[aoc(day2, part2)]
fn part2(input: &[usize]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
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
            assert_eq!(part1(&parse(input)), expected);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
