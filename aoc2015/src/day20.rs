use aoc_runner_derive::{aoc, aoc_generator};

type Presents = u32;
type HouseNumber = u32;

#[aoc_generator(day20)]
fn parse(input: &str) -> Presents {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
fn part1(input: &Presents) -> HouseNumber {
    // house numbers
    (1..)
        .map(|hn| {
            // elves bearing presents
            let prs: Presents = (1..=hn).filter(|e| hn % e == 0).map(|e| e * 10).sum();
            (hn, prs)
        })
        .inspect(|(hn, prs)| {
            eprintln!("House: {hn}, presents: {prs}");
        })
        .skip_while(|(_, prs)| prs < input)
        .map(|(hn, _)| hn)
        .next()
        .unwrap()
}

#[derive(Debug)]
struct Elf {
    number: u32,
    deliveries: u32,
}

impl Elf {
    fn new(number: u32) -> Self {
        Self {
            number,
            deliveries: 50,
        }
    }

    fn deliver_presents(&mut self) -> u32 {
        self.deliveries -= 1;
        self.number * 11
    }
}

#[aoc(day20, part2)]
fn part2(input: &Presents) -> HouseNumber {
    // house numbers
    let mut house_number = 1;
    let mut elves = vec![];

    loop {
        elves.push(Elf::new(house_number));

        // elves bearing presents
        let mut prs = 0;
        for elf in &mut elves {
            if house_number % elf.number == 0 {
                prs += elf.deliver_presents();
            }
        }

        elves.retain(|e| e.deliveries > 0);

        eprintln!("House: {house_number}, presents: {prs}");

        if prs >= *input {
            break house_number;
        }

        house_number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (hn, prs) in [(1, 10), (4, 70)] {
            assert_eq!(hn, part1(&prs));
        }
    }

    #[test]
    fn part2_example() {
        for (hn, prs) in [(1, 11), (4, 77)] {
            assert_eq!(hn, part2(&prs));
        }
    }
}
