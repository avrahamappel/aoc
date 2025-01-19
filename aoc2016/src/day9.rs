use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

#[aoc(day9, part1)]
fn part1(input: &[u8]) -> usize {
    let mut decompressed = Vec::new();
    let mut idx = 0;
    while idx < input.len() {
        if input[idx] == b'(' {
            let mut len_str = vec![];
            idx += 1;
            while input[idx] != b'x' {
                len_str.push(input[idx]);
                idx += 1;
            }
            let mut rpt_str = vec![];
            idx += 1;
            while input[idx] != b')' {
                rpt_str.push(input[idx]);
                idx += 1;
            }
            idx += 1;
            let len: usize = String::from_utf8(len_str).unwrap().parse().unwrap();
            let rpt: usize = String::from_utf8(rpt_str).unwrap().parse().unwrap();
            decompressed.extend(input[idx..idx + len].repeat(rpt));
            idx += len;
        } else {
            decompressed.push(input[idx]);
            idx += 1;
        }
    }
    println!("{}", std::str::from_utf8(&decompressed).unwrap());
    decompressed.len()
}

#[aoc(day9, part2)]
fn part2(input: &[u8]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (input, output) in [
            ("ADVENT", 6),
            ("A(1x5)BC", 7),
            ("(3x3)XYZ", 9),
            ("A(2x2)BCD(2x2)EFG", 11),
            ("(6x1)(1x3)A", 6),
            ("X(8x2)(3x3)ABCY", 18),
        ] {
            eprintln!("input: {input}");
            assert_eq!(part1(&parse(input)), output);
        }
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
