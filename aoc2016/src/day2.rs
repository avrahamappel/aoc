use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
enum Dir {
    U,
    D,
    R,
    L,
}

type Dirs = Vec<Dir>;
type Lines = Vec<Dirs>;
type Key = char;

struct Keypad {
    ln_len: usize,
    vec: Vec<Key>,
}

impl Keypad {
    fn new(s: &str, ln_len: usize) -> Self {
        let vec = s
            .lines()
            .filter(|l| !l.is_empty())
            .flat_map(|l| l.chars())
            .collect();

        Self { ln_len, vec }
    }

    /// Find the index of a key character
    fn index_of(&self, key: Key) -> usize {
        self.vec.iter().position(|k| k == &key).unwrap()
    }

    /// Move from the specified position in the specified direction.
    /// Returns the value of the new position and its index
    fn move_from(&self, pos: usize, dir: Dir) -> (Key, usize) {
        let new_pos = {
            match dir {
                Dir::U => {
                    if pos < self.ln_len {
                        pos
                    } else {
                        pos - self.ln_len
                    }
                }
                Dir::D => {
                    if pos >= self.vec.len() - self.ln_len {
                        pos
                    } else {
                        pos + self.ln_len
                    }
                }
                Dir::R => {
                    if pos % self.ln_len == self.ln_len - 1 {
                        pos
                    } else {
                        pos + 1
                    }
                }
                Dir::L => {
                    if pos % self.ln_len == 0 {
                        pos
                    } else {
                        pos - 1
                    }
                }
            }
        };
        let key = self.vec[new_pos];
        if key == ' ' {
            return (self.vec[pos], pos);
        }

        (self.vec[new_pos], new_pos)
    }

    /// Follow a set of directions to arrive at a final key
    fn follow_dirs(&self, dirs: &[Dir], init_key: Key) -> Key {
        let mut key = init_key;
        let mut pos = self.index_of(key);

        for dir in dirs {
            (key, pos) = self.move_from(pos, *dir);
        }

        key
    }

    /// Follow multiple sets of directions to get a multidigit code
    fn follow_dir_lists(&self, dir_lists: &[Dirs]) -> String {
        let initial_key = '5';
        dir_lists
            .iter()
            .fold(vec![], |mut digits, list| {
                let key = digits.last().copied().unwrap_or(initial_key);
                let new_key = self.follow_dirs(list, key);
                digits.push(new_key);
                digits
            })
            .iter()
            .collect()
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Lines {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .filter_map(|c| match c {
                    'U' => Some(Dir::U),
                    'D' => Some(Dir::D),
                    'R' => Some(Dir::R),
                    'L' => Some(Dir::L),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Lines) -> String {
    let keypad = Keypad::new(
        "
123
456
789
",
        3,
    );

    keypad.follow_dir_lists(input)
}

#[aoc(day2, part2)]
fn part2(input: &Lines) -> String {
    let keypad = Keypad::new(
        "
  1  
 234 
56789
 ABC 
  D  
",
        5,
    );

    keypad.follow_dir_lists(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "ULL
                 RRDDD
                 LURDL
                 UUUUD"
            )),
            "1985"
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "ULL
                 RRDDD
                 LURDL
                 UUUUD"
            )),
            "5DB3"
        );
    }
}
