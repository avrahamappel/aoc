use std::collections::{HashMap, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

type BotId = u32;
type OutputId = u32;
type Chip = u32;

#[derive(Clone, Copy)]
enum Dest {
    Bot(BotId),
    Output(OutputId),
}

impl Dest {
    fn new(kind: &str, id: &str) -> Self {
        let id = id.parse().unwrap();
        if kind == "output" {
            Self::Output(id)
        } else {
            Self::Bot(id)
        }
    }

    fn send(self, chip: Chip, factory: &mut Factory) -> Option<BotAction> {
        match self {
            Dest::Bot(bot_id) => {
                let bot = factory.get_or_insert_bot(bot_id);
                bot.receive(chip)
            }
            Dest::Output(op_id) => {
                let output = factory.get_or_insert_output(op_id);
                output.chips.push(chip);
                None
            }
        }
    }

    fn display(self) -> String {
        match self {
            Self::Bot(b) => format!("bot {b}"),
            Self::Output(o) => format!("output {o}"),
        }
    }
}

#[derive(Clone, Copy)]
struct ToBot {
    value: Chip,
    bot: BotId,
}

#[derive(Clone, Copy)]
struct FromBot {
    bot: BotId,
    low: Dest,
    high: Dest,
}

#[derive(Clone, Copy)]
enum Instruction {
    ToBot(ToBot),
    FromBot(FromBot),
}

type Input = Vec<Instruction>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut words = l.trim().split_ascii_whitespace();
            match words.next().unwrap() {
                "value" => {
                    let value = words.next().unwrap().parse().unwrap();
                    let bot = words.nth(3).unwrap().parse().unwrap();
                    Instruction::ToBot(ToBot { value, bot })
                }
                "bot" => {
                    let bot = words.next().unwrap().parse().unwrap();
                    let low = {
                        let kind = words.nth(3).unwrap();
                        let id = words.next().unwrap();
                        Dest::new(kind, id)
                    };
                    let high = {
                        let kind = words.nth(3).unwrap();
                        let id = words.next().unwrap();
                        Dest::new(kind, id)
                    };
                    Instruction::FromBot(FromBot { bot, low, high })
                }
                _ => unimplemented!(),
            }
        })
        .collect()
}

struct BotAction {
    low: (Chip, Dest),
    high: (Chip, Dest),
}

impl BotAction {
    fn send_low(&self, factory: &mut Factory) -> Option<BotAction> {
        self.low.1.send(self.low.0, factory)
    }

    fn send_high(&self, factory: &mut Factory) -> Option<BotAction> {
        self.high.1.send(self.high.0, factory)
    }
}

struct Bot {
    id: BotId,
    chips: Vec<Chip>,
    instr: FromBot,
}

impl Bot {
    fn new(id: BotId, instr: FromBot) -> Self {
        Self {
            id,
            instr,
            chips: vec![],
        }
    }

    fn receive(&mut self, chip: Chip) -> Option<BotAction> {
        print!("Bot {} receives {chip}", self.id);
        self.chips.push(chip);
        if self.chips.len() < 2 {
            println!();
            None
        } else {
            let high = self.chips.iter().copied().max().unwrap();
            let low = self.chips.iter().copied().min().unwrap();

            self.chips.clear();

            println!(
                ", and passes low value {low} to {}, high value {high} to {}",
                self.instr.low.display(),
                self.instr.high.display()
            );
            Some(BotAction {
                low: (low, self.instr.low),
                high: (high, self.instr.high),
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Output {
    id: OutputId,
    chips: Vec<Chip>,
}

impl Output {
    fn new(id: OutputId) -> Self {
        Self { id, chips: vec![] }
    }
}

#[derive(Default)]
struct Factory {
    bots: HashMap<BotId, Bot>,
    outputs: HashMap<OutputId, Output>,
    instrs: Input,
}

impl Factory {
    fn new(input: &Input) -> Self {
        Self {
            instrs: input.to_owned(),
            ..Self::default()
        }
    }

    fn get_or_insert_bot(&mut self, bot_id: BotId) -> &mut Bot {
        self.bots.entry(bot_id).or_insert_with(|| {
            let instr = self
                .instrs
                .iter()
                .find_map(|i| match i {
                    Instruction::FromBot(fb) if fb.bot == bot_id => Some(fb),
                    _ => None,
                })
                .expect(&format!("Should be an instr for bot [{}]", bot_id));
            Bot::new(bot_id, *instr)
        })
    }

    fn get_or_insert_output(&mut self, output_id: OutputId) -> &mut Output {
        self.outputs
            .entry(output_id)
            .or_insert(Output::new(output_id))
    }

    fn run(&mut self) -> u32 {
        for i in 0..self.instrs.len() {
            match self.instrs[i] {
                Instruction::ToBot(tb) => {
                    let bot = self.get_or_insert_bot(tb.bot);

                    if let Some(action) = bot.receive(tb.value) {
                        //////////////////////
                        // RETURN CONDITION //
                        //////////////////////
                        /// for some reason never got called, but i found it in the output manually
                        if action.low.0 == 17 && action.high.0 == 61 {
                            return bot.id;
                        }

                        let mut action_queue = VecDeque::from([action]);

                        while let Some(axn) = action_queue.pop_front() {
                            if let Some(low_axn) = axn.send_low(self) {
                                action_queue.push_back(low_axn);
                            }
                            if let Some(high_axn) = axn.send_high(self) {
                                action_queue.push_back(high_axn);
                            }
                        }
                    }
                }
                Instruction::FromBot(_fb) => {
                    // do we need to do anything here?
                }
            }
        }
        0
    }
}

#[aoc(day10, part1)]
fn part1(input: &Input) -> u32 {
    Factory::new(input).run()
}

#[aoc(day10, part2)]
fn part2(input: &Input) -> String {
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part1_example() {
        let mut factory = Factory::new(&parse(
            "value 5 goes to bot 2
             bot 2 gives low to bot 1 and high to bot 0
             value 3 goes to bot 1
             bot 1 gives low to output 1 and high to bot 0
             bot 0 gives low to output 2 and high to output 0
             value 2 goes to bot 2",
        ));
        factory.run();

        assert_eq!(
            &Output {
                id: 0,
                chips: vec![5],
            },
            factory.outputs.get(&0).unwrap()
        );
        assert_eq!(
            &Output {
                id: 1,
                chips: vec![2]
            },
            factory.outputs.get(&1).unwrap()
        );
        assert_eq!(
            &Output {
                id: 2,
                chips: vec![3]
            },
            factory.outputs.get(&2).unwrap()
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
