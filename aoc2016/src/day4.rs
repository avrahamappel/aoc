use aoc_runner_derive::{aoc, aoc_generator};

struct Room {
    name: String,
    sector: u32,
    checksum: Vec<char>,
}

impl Room {
    fn is_valid(&self) -> bool {
        let mut chars = self.name.chars().filter(char::is_ascii_alphabetic).fold(
            Vec::new(),
            |mut map, char| {
                if let Some((_, v)) = map.iter_mut().find(|(c, _)| *c == char) {
                    *v += 1;
                } else {
                    map.push((char, 1));
                }
                map
            },
        );

        chars.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        self.checksum
            .iter()
            .zip(chars.iter().map(|c| c.0))
            .all(|(c1, c2)| *c1 == c2)
    }

    fn decrypt_name(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    let offset: u32 = b'a'.into();
                    char::from_u32(((c as u32) - offset + self.sector) % 26 + offset).unwrap()
                } else {
                    ' '
                }
            })
            .collect()
    }
}

type Input = Vec<Room>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let l = l.trim();
            let (l, checksum) = l.trim_end_matches(']').split_once('[').unwrap();
            let checksum = checksum.chars().collect();
            let (l, sector) = l.rsplit_once('-').unwrap();
            let sector = sector.parse().unwrap();
            let name = l.to_string();
            Room {
                name,
                sector,
                checksum,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> u32 {
    input
        .iter()
        .filter(|r| r.is_valid())
        .map(|r| r.sector)
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &Input) -> usize {
    for room in input {
        println!("{}: {}", room.sector, room.decrypt_name());
    }
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "aaaaa-bbb-z-y-x-123[abxyz]
                 a-b-c-d-e-f-g-h-987[abcde]
                 not-a-real-room-404[oarel]
                 totally-real-room-200[decoy]"
            )),
            1514
        );
    }

    #[test]
    fn part2_example() {
        let room = Room {
            name: "qzmt-zixmtkozy-ivhz".into(),
            sector: 343,
            checksum: vec![],
        };

        assert_eq!(room.decrypt_name(), String::from("very encrypted name"));
    }
}
