use aoc_runner_derive::{aoc, aoc_generator};

enum Dir {
    R,
    L,
}

struct Rotation {
    dir: Dir,
    num: usize,
}

struct Dial {
    current_position: usize,
    passed_zero_times: usize,
}

impl Dial {
    fn new() -> Self {
        Dial {
            current_position: 50,
            passed_zero_times: 0,
        }
    }
    fn rotate(&mut self, rotation: &Rotation) {
        match rotation.dir {
            Dir::R => {
                self.current_position = (self.current_position + rotation.num) % 100;
                eprintln!(
                    "The dial is rotated R{} to point at {}",
                    rotation.num, self.current_position
                );
            }
            Dir::L => {
                let normalized_num = rotation.num % 100;
                let new_position = if normalized_num > self.current_position {
                    100 + self.current_position - normalized_num
                } else {
                    self.current_position - normalized_num
                };
                self.current_position = new_position;
                eprintln!(
                    "The dial is rotated L{} to point at {}",
                    rotation.num, self.current_position
                );
            }
        }

        if self.current_position == 0 {
            self.passed_zero_times += 1;
        }

        eprintln!("Passed zero {} times", self.passed_zero_times);
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<Rotation> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (dir_str, num_str) = l.trim().split_at(1);

            let dir = match dir_str {
                "R" => Dir::R,
                "L" => Dir::L,
                _ => panic!("Invalid direction"),
            };

            let num = num_str.parse::<usize>().expect("Invalid number");

            Rotation { dir, num }
        })
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Rotation]) -> usize {
    let mut dial = Dial::new();

    for rotation in input {
        dial.rotate(rotation);
    }

    dial.passed_zero_times
}

#[aoc(day1, part2)]
fn part2(input: &[Rotation]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "
                L68
                L30
                R48
                L5
                R60
                L55
                L1
                L99
                R14
                L82
                "
            )),
            3
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
