use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type BagCount = usize;
type BagColor = String;
type Rule = HashMap<BagColor, BagCount>;
type Input = HashMap<BagColor, Rule>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (rule_str, bags_str) = l.trim().split_once(" contain ").unwrap();
            let color = rule_str.trim_end_matches(" bags").to_string();
            let bags = bags_str
                .trim_end_matches('.')
                .split(", ")
                .filter_map(|b| {
                    let (qty_str, clr_str) = b
                        .trim_end_matches('s')
                        .trim_end_matches(" bag")
                        .split_once(' ')
                        .unwrap();
                    let quantity = qty_str.parse().ok()?;
                    let color = clr_str.to_string();
                    Some((color, quantity))
                })
                .collect();
            (color, bags)
        })
        .collect()
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> usize {
    let mut checked_colors = HashMap::new();

    for color in input.keys() {
        fn bag_can_hold_shiny_gold(
            color: &BagColor,
            rules: &Input,
            checked_colors: &mut HashMap<BagColor, bool>,
        ) -> bool {
            if let Some(can_hold) = checked_colors.get(color) {
                return *can_hold;
            }
            let bags = rules.get(color).unwrap();
            let can_hold = bags.keys().any(|bag| {
                *bag == "shiny gold" || bag_can_hold_shiny_gold(bag, rules, checked_colors)
            });
            checked_colors.insert(color.clone(), can_hold);
            can_hold
        }

        let can_hold = bag_can_hold_shiny_gold(color, input, &mut checked_colors);
        checked_colors.insert(color.clone(), can_hold);
    }

    checked_colors.into_iter().filter(|(_, b)| *b).count()
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> usize {
    fn count_bags(bag: &BagColor, rules: &Input, i: usize) -> usize {
        let indent = "  ".repeat(i);
        eprintln!("{indent}Counting contents of bag [{bag}]");
        let rule = rules.get(bag).unwrap();
        if rule.is_empty() {
            eprintln!("{indent}No bags");
            1
        } else {
            let total: usize = rule
                .iter()
                .map(|(color, count)| {
                    eprintln!("{indent}color: {color}, count: {count}");
                    count * count_bags(color, rules, i + 1)
                })
                .sum();
            eprintln!("{indent}bag [{bag}] total: {total}");
            total + 1
        }
    }
    count_bags(&String::from("shiny gold"), input, 0) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
                 dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                 bright white bags contain 1 shiny gold bag.
                 muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                 shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                 dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                 vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                 faded blue bags contain no other bags.
                 dotted black bags contain no other bags."
            )),
            4
        );
    }

    #[test]
    fn part2_example() {
        for (input, output) in [
            (
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
                 dark orange bags contain 3 bright white bags, 4 muted yellow bags.
                 bright white bags contain 1 shiny gold bag.
                 muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
                 shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
                 dark olive bags contain 3 faded blue bags, 4 dotted black bags.
                 vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
                 faded blue bags contain no other bags.
                 dotted black bags contain no other bags.",
                32,
            ),
            (
                "shiny gold bags contain 2 dark red bags.
                 dark red bags contain 2 dark orange bags.
                 dark orange bags contain 2 dark yellow bags.
                 dark yellow bags contain 2 dark green bags.
                 dark green bags contain 2 dark blue bags.
                 dark blue bags contain 2 dark violet bags.
                 dark violet bags contain no other bags.",
                126,
            ),
        ] {
            assert_eq!(part2(&parse(input)), output);
        }
    }
}
