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
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
