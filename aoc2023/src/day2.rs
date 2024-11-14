use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Default)]
struct Show {
    red: u32,
    green: u32,
    blue: u32,
}

impl Show {
    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
}

struct Game {
    id: u32,
    shows: Vec<Show>,
}

type Input = Vec<Game>;

#[aoc_generator(day2)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter_map(|l| {
            let (game_str, shows_str) = l.trim().split_once(": ")?;

            let id = game_str.trim_start_matches("Game ").parse().ok()?;
            let shows = shows_str
                .split("; ")
                .map(|show_str| {
                    let mut show = Show::default();
                    for color_str in show_str.split(", ") {
                        if let Some((n, color)) = color_str.split_once(' ') {
                            if let Ok(count) = n.parse::<u32>() {
                                match color {
                                    "red" => {
                                        show.red += count;
                                    }
                                    "green" => {
                                        show.green += count;
                                    }
                                    "blue" => {
                                        show.blue += count;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    show
                })
                .collect();

            Some(Game { id, shows })
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Input) -> u32 {
    let sample = Show {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .iter()
        .filter(|g| g.shows.iter().all(|s| sample.contains(s)))
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
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
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                 Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                 Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                 Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                 Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )),
            8
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
