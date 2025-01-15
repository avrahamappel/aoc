use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use strum::VariantArray;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, VariantArray)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(self) -> u32 {
        match self {
            Self::MagicMissile => 53,
            Self::Drain => 73,
            Self::Shield => 113,
            Self::Poison => 173,
            Self::Recharge => 229,
        }
    }

    fn turns(self) -> u32 {
        match self {
            Self::Poison => 6,
            Self::Shield => 6,
            Self::Recharge => 5,
            _ => 0,
        }
    }

    fn instant_effect(self, battle: &mut Battle) {
        match self {
            Self::MagicMissile => {
                battle.boss.hp = battle.boss.hp.saturating_sub(4);
            }
            Self::Drain => {
                battle.boss.hp = battle.boss.hp.saturating_sub(2);
                battle.wizard.hp += 2;
            }
            _ => {}
        }
    }

    fn apply_effect(self, battle: &mut Battle) {
        match self {
            Self::Poison => {
                battle.boss.hp = battle.boss.hp.saturating_sub(3);
            }
            Self::Recharge => {
                battle.wizard.mana += 101;
            }
            _ => {}
        }
    }
}

#[derive(Default, Clone)]
struct Wizard {
    hp: u32,
    mana: u32,
    mana_spent: u32,
    current_effects: HashMap<Spell, u32>,
    spell_history: Vec<Spell>,
}

impl Wizard {
    fn new() -> Self {
        Self {
            hp: 50,
            mana: 500,
            ..Default::default()
        }
    }

    fn can_cast(&self, spell: Spell) -> bool {
        !self.current_effects.contains_key(&spell) && self.mana >= spell.cost()
    }

    fn cast(&mut self, spell: Spell) {
        assert!(self.can_cast(spell));

        self.current_effects.insert(spell, spell.turns());
        self.spell_history.push(spell);
        self.mana = self.mana.saturating_sub(spell.cost());
        self.mana_spent += spell.cost();
    }
}

#[derive(Clone, Copy)]
struct Boss {
    hp: u32,
    damage: u32,
}

impl Boss {
    fn attack(self, wizard: &mut Wizard) {
        let damage = if wizard.current_effects.contains_key(&Spell::Shield) {
            1.max(self.damage.saturating_sub(7))
        } else {
            self.damage
        };
        wizard.hp = wizard.hp.saturating_sub(damage);
    }
}

#[derive(Debug, PartialEq)]
enum BattleStatus {
    WizardWon,
    BossWon,
    Continuing,
}

#[derive(Clone)]
struct Battle {
    wizard: Wizard,
    boss: Boss,
}

impl Battle {
    fn new(boss: Boss) -> Self {
        let wizard = Wizard::new();
        Self { wizard, boss }
    }

    fn round(&mut self, spell: Spell) -> BattleStatus {
        println!("-- Player turn --");
        println!(
            "- Player has {} hit points, {} mana",
            self.wizard.hp, self.wizard.mana
        );
        println!("- Boss has {} hit points", self.boss.hp);
        self.apply_spell_effects();
        if self.boss.hp == 0 {
            println!("Boss killed!");
            return BattleStatus::WizardWon;
        }
        println!("Player casts {spell:?}");
        self.wizard.cast(spell);
        spell.instant_effect(self);
        if self.boss.hp == 0 {
            println!("Boss killed!");
            return BattleStatus::WizardWon;
        }

        println!("-- Boss turn --");
        println!(
            "- Player has {} hit points, {} mana",
            self.wizard.hp, self.wizard.mana
        );
        println!("- Boss has {} hit points", self.boss.hp);
        self.apply_spell_effects();
        if self.boss.hp == 0 {
            println!("Boss killed!");
            return BattleStatus::WizardWon;
        }
        println!("Boss attacks");
        self.boss.attack(&mut self.wizard);
        if self.wizard.hp == 0 {
            println!("Player killed!");
            return BattleStatus::BossWon;
        }

        BattleStatus::Continuing
    }

    fn apply_spell_effects(&mut self) {
        for spell in Spell::VARIANTS {
            if self.wizard.current_effects.contains_key(spell) {
                spell.apply_effect(self);
            }
        }

        // Filter out expired effects
        self.wizard.current_effects = self
            .wizard
            .current_effects
            .iter()
            .filter(|kv| *kv.1 > 0)
            .map(|(s, t)| (*s, *t - 1))
            .collect();
    }
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Boss {
    let mut hp = 0;
    let mut damage = 0;
    for line in input.lines() {
        let (field, amt) = line.trim().split_once(": ").unwrap();
        let amount: u32 = amt.parse().unwrap();
        match field {
            "Hit Points" => {
                hp = amount;
            }
            "Damage" => {
                damage = amount;
            }
            _ => {}
        }
    }
    Boss { hp, damage }
}

#[aoc(day22, part1)]
fn part1(boss: &Boss) -> u32 {
    let mut battles = VecDeque::from([Battle::new(*boss)]);
    let mut best_wizard: Option<Wizard> = None;

    while let Some(battle) = battles.pop_front() {
        for spell in Spell::VARIANTS {
            if !battle.wizard.can_cast(*spell) {
                continue;
            }
            let mut next_battle = battle.clone();
            let result = next_battle.round(*spell);

            print!("Used spells:");
            for s in &next_battle.wizard.spell_history {
                print!("{s:?} ");
            }
            println!();

            // If we're no longer the best wizard, don't continue this battle
            if let Some(ref bw) = best_wizard {
                if bw.mana_spent <= next_battle.wizard.mana_spent {
                    break;
                }
            }

            match result {
                BattleStatus::BossWon => {}
                BattleStatus::WizardWon => {
                    eprintln!("wizard won!");
                    best_wizard = Some(next_battle.wizard);
                }
                BattleStatus::Continuing => {
                    battles.push_back(next_battle);
                }
            }
        }
        //for b in &battles {
        //    eprintln!("mana spent: {}", b.wizard.mana_spent);
        //}
        if let Some(ref bw) = best_wizard {
            eprintln!("best wizard: {}", bw.mana_spent);
        }
    }

    // return wizard with least mana_spent
    best_wizard.unwrap().mana_spent
}

#[aoc(day22, part2)]
fn part2(input: &Boss) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battle1() {
        let mut battle = Battle {
            wizard: Wizard {
                hp: 10,
                mana: 250,
                ..Wizard::default()
            },
            boss: Boss { hp: 13, damage: 8 },
        };
        battle.round(Spell::Poison);
        assert_eq!(BattleStatus::WizardWon, battle.round(Spell::MagicMissile));
    }

    #[test]
    fn test_battle2() {
        let mut battle = Battle {
            wizard: Wizard {
                hp: 10,
                mana: 250,
                ..Wizard::default()
            },
            boss: Boss { hp: 14, damage: 8 },
        };
        battle.round(Spell::Recharge);
        battle.round(Spell::Shield);
        battle.round(Spell::Drain);
        battle.round(Spell::Poison);
        assert_eq!(BattleStatus::WizardWon, battle.round(Spell::MagicMissile));
    }

    //#[test]
    //fn part2_example() {
    //    assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    //}
}
