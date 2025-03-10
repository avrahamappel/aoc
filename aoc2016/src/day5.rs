use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day5, part1)]
fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut i = 0;

    while password.len() < 8 {
        let val = format!("{input}{i}");
        eprint!("string:    {val}");
        let digest = md5::compute(val);
        let hash = format!("{digest:x}");
        eprintln!("hash:    {hash}");
        if hash.starts_with("00000") {
            password.push(hash.chars().nth(5).unwrap());
            eprintln!("password:  {password}");
        }
        i += 1;
    }

    password
}

#[aoc(day5, part2)]
fn part2(input: &str) -> String {
    let mut password = String::from("________");
    let mut i = 0;

    while password.contains('_') {
        let val = format!("{input}{i}");
        eprint!("string:    {val}     ");
        let digest = md5::compute(val);
        let hash = format!("{digest:x}");
        //eprintln!("hash:    {hash}");
        if hash.starts_with("00000") {
            let pos = hash.chars().nth(5).unwrap();
            if pos.is_ascii_digit() {
                let pos = pos.to_digit(10).unwrap();
                if pos < 8 && password.chars().nth(pos as usize).unwrap() == '_' {
                    password.replace_range(
                        pos as usize..=pos as usize,
                        &hash.chars().nth(6).unwrap().to_string(),
                    );
                }
            }
        }
        eprintln!("password:  {password}");
        i += 1;
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("abc")), "18f47a30");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("abc")), "05ace8e3");
    }
}
