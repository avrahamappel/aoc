use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

type Input = Vec<Claim>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut tokens = l.trim().split_ascii_whitespace();
            let id = tokens
                .next()
                .unwrap()
                .trim_start_matches('#')
                .parse()
                .unwrap();
            let _ = tokens.next();
            let mut pos = tokens
                .next()
                .unwrap()
                .trim_end_matches(':')
                .split(',')
                .map(str::parse)
                .filter_map(Result::ok);
            let left = pos.next().unwrap();
            let top = pos.next().unwrap();
            let mut dims = tokens
                .next()
                .unwrap()
                .split('x')
                .map(str::parse)
                .filter_map(Result::ok);
            let width = dims.next().unwrap();
            let height = dims.next().unwrap();
            Claim {
                id,
                left,
                top,
                width,
                height,
            }
        })
        .collect()
}

fn map_claim_ids_to_cells(input: &Input) -> HashMap<(u32, u32), Vec<u32>> {
    // map of cell to claim ids
    let mut map = HashMap::new();
    for claim in input {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                map.entry((x, y))
                    .and_modify(|claims: &mut Vec<_>| claims.push(claim.id))
                    .or_insert(vec![claim.id]);
            }
        }
    }

    map
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> usize {
    // count of cells with more than one id
    map_claim_ids_to_cells(input)
        .iter()
        .filter(|(_, cs)| cs.len() > 1)
        .count()
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> u32 {
    let multiples: HashSet<_> = map_claim_ids_to_cells(input)
        .into_values()
        .filter(|cs| cs.len() > 1)
        .flatten()
        .collect();

    input
        .iter()
        .find(|c| !multiples.contains(&c.id))
        .unwrap()
        .id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "#1 @ 1,3: 4x4
                 #2 @ 3,1: 4x4
                 #3 @ 5,5: 2x2"
            )),
            4
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "#1 @ 1,3: 4x4
                 #2 @ 3,1: 4x4
                 #3 @ 5,5: 2x2"
            )),
            3
        );
    }
}
