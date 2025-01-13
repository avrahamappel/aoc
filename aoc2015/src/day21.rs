use std::collections::VecDeque;
use std::num::Saturating;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Weapon {
    name: &'static str,
    damage: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Armor {
    name: &'static str,
    armor: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ring {
    name: &'static str,
    armor: u32,
    damage: u32,
}

#[derive(Clone, Copy)]
enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Ring(Ring),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Fighter {
    spent: u32,
    hp: Saturating<u32>,
    armor_: Option<Armor>,
    weapon: Weapon,
    ring1: Option<Ring>,
    ring2: Option<Ring>,
}

impl Fighter {
    fn new() -> Self {
        Self {
            hp: Saturating(100),
            ..Default::default()
        }
    }

    fn damage(self) -> u32 {
        let mut damage = self.weapon.damage;
        if let Some(r1) = self.ring1 {
            damage += r1.damage;
        }
        if let Some(r2) = self.ring2 {
            damage += r2.damage;
        }
        damage
    }

    fn armor(self) -> u32 {
        let mut armor = 0;
        if let Some(a) = self.armor_ {
            armor += a.armor;
        }
        if let Some(r1) = self.ring1 {
            armor += r1.armor;
        }
        if let Some(r2) = self.ring2 {
            armor += r2.armor;
        }
        armor
    }

    fn buy(mut self, price: u32, item: Item) -> Option<Self> {
        self.spent += price;
        match item {
            Item::Weapon(w) if self.weapon.damage == 0 => {
                self.weapon = w;
                Some(self)
            }
            Item::Armor(a) if self.armor_.is_none() => {
                self.armor_ = Some(a);
                Some(self)
            }
            Item::Ring(r) => {
                if self.ring1.is_none() {
                    self.ring1 = Some(r);
                    Some(self)
                } else if self.ring2.is_none() && self.ring1.unwrap() != r {
                    self.ring2 = Some(r);
                    Some(self)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn fight(mut self, mut other: Self) -> bool {
        loop {
            self.attack(&mut other);
            if other.hp == Saturating(0) {
                break true;
            }
            other.attack(&mut self);
            if self.hp == Saturating(0) {
                break false;
            }
        }
    }

    fn attack(self, other: &mut Self) {
        dbg!(self.damage(), other.armor());
        other.hp -= 1.max(self.damage().saturating_sub(other.armor()));
        dbg!(other.hp);
    }
}

const STORE: [(u32, Item); 16] = [
    // Weapons:
    (
        8,
        Item::Weapon(Weapon {
            name: "Dagger",
            damage: 4,
        }),
    ),
    (
        10,
        Item::Weapon(Weapon {
            name: "Shortsword",
            damage: 5,
        }),
    ),
    (
        25,
        Item::Weapon(Weapon {
            name: "Warhammer",
            damage: 6,
        }),
    ),
    (
        40,
        Item::Weapon(Weapon {
            name: "Longsword",
            damage: 7,
        }),
    ),
    (
        74,
        Item::Weapon(Weapon {
            name: "Greataxe",
            damage: 8,
        }),
    ),
    // Armor:
    (
        13,
        Item::Armor(Armor {
            name: "Leather",
            armor: 1,
        }),
    ),
    (
        31,
        Item::Armor(Armor {
            name: "Chainmail",
            armor: 2,
        }),
    ),
    (
        53,
        Item::Armor(Armor {
            name: "Splintmail",
            armor: 3,
        }),
    ),
    (
        75,
        Item::Armor(Armor {
            name: "Bandedmail",
            armor: 4,
        }),
    ),
    (
        102,
        Item::Armor(Armor {
            name: "Platemail",
            armor: 5,
        }),
    ),
    // Rings:
    (
        25,
        Item::Ring(Ring {
            name: "Damage +1",
            damage: 1,
            armor: 0,
        }),
    ),
    (
        50,
        Item::Ring(Ring {
            name: "Damage +2",
            damage: 2,
            armor: 0,
        }),
    ),
    (
        100,
        Item::Ring(Ring {
            name: "Damage +3",
            damage: 3,
            armor: 0,
        }),
    ),
    (
        20,
        Item::Ring(Ring {
            name: "Defense +1",
            damage: 0,
            armor: 1,
        }),
    ),
    (
        40,
        Item::Ring(Ring {
            name: "Defense +2",
            damage: 0,
            armor: 2,
        }),
    ),
    (
        80,
        Item::Ring(Ring {
            name: "Defense +3",
            damage: 0,
            armor: 3,
        }),
    ),
];

#[aoc_generator(day21)]
fn parse(input: &str) -> Fighter {
    let mut fighter = Fighter::new();
    for line in input.lines() {
        let (field, amt) = line.trim().split_once(": ").unwrap();
        let amount = amt.parse().unwrap();
        match field {
            "Hit Points" => {
                fighter.hp = Saturating(amount);
            }
            "Damage" => {
                fighter.weapon = Weapon {
                    name: "",
                    damage: amount,
                };
            }
            "Armor" => {
                fighter.armor_ = Some(Armor {
                    name: "",
                    armor: amount,
                });
            }
            _ => {}
        }
    }
    fighter
}

#[aoc(day21, part1)]
fn part1(boss: &Fighter) -> u32 {
    let mut fighters = vec![];
    let mut queue = VecDeque::from([Fighter::new()]);

    while let Some(fighter) = queue.pop_front() {
        dbg!(fighter);
        if fighters.contains(&fighter) {
            continue;
        }
        if fighter.fight(*boss) {
            fighters.push(fighter);
        }
        for (price, item) in STORE {
            if let Some(f) = fighter.buy(price, item) {
                queue.push_back(f);
            }
        }
    }

    fighters.iter().min_by_key(|f| f.spent).unwrap().spent
}

#[aoc(day21, part2)]
fn part2(boss: &Fighter) -> u32 {
    let mut fighters = vec![];
    let mut queue = VecDeque::from([Fighter::new()]);

    while let Some(fighter) = queue.pop_front() {
        if fighters.contains(&fighter) {
            continue;
        }
        dbg!(fighter);
        if fighter.weapon.damage != 0 && !fighter.fight(*boss) {
            fighters.push(fighter);
        }
        for (price, item) in STORE {
            if let Some(f) = fighter.buy(price, item) {
                queue.push_back(f);
            }
        }
    }

    fighters.iter().max_by_key(|f| f.spent).unwrap().spent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        let player = parse(
            "Hit Points: 8
            Damage: 5
            Armor: 5",
        );
        let boss = parse(
            "Hit Points: 12
            Damage: 7
            Armor: 2",
        );
        assert!(player.fight(boss));
    }
}
