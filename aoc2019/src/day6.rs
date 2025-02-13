use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Target = String;
type Object = String;
type Input = Vec<(Target, Object)>;

#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (target, object) = l.trim().split_once(')').unwrap();
            (target.to_string(), object.to_string())
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u32 {
    let mut orbits: HashMap<&str, u32> = input.iter().map(|kv| (kv.1.as_str(), 0)).collect();
    let mut searches = std::collections::VecDeque::from(["COM"]);
    while let Some(s) = searches.pop_front() {
        // find all objects that orbit this target
        let objs: Vec<_> = input
            .iter()
            .filter(|(tar, _obj)| tar == s)
            .map(|to| to.1.as_str())
            .collect();
        for o in &objs {
            // increment objects orbit count by 1 plus the orbit count of the target
            let target_orbit_count = orbits.get(s).copied().unwrap_or(0);
            orbits.entry(o).and_modify(|v| *v += 1 + target_orbit_count);
        }
        searches.extend(objs);
    }

    orbits.values().sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "COM)B
                 B)C
                 C)D
                 D)E
                 E)F
                 B)G
                 G)H
                 D)I
                 E)J
                 J)K
                 K)L"
            )),
            42
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
