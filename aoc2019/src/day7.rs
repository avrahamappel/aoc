use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::{Rc, Weak},
};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::intcode::{Intcode, State};

type Input = Vec<i64>;

#[aoc_generator(day7)]
fn parse(input: &str) -> Input {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &Input) -> i64 {
    let mut max_output = 0;
    for permutation in (0..=4).permutations(5) {
        let mut prev_output = 0;
        for setting in permutation {
            //dbg!(setting, max_output, prev_output);
            let mut prg = Intcode::new(input.clone());
            // first setting then previous output or 0
            let inp = [setting, prev_output];
            let mut output = 0;
            for i in inp {
                let state = prg.run(Some(i));
                match state {
                    State::Output(o) => output = o,
                    State::Halted => break,
                    State::NeedsInput => {}
                }
            }
            if output > max_output {
                max_output = output;
            }
            prev_output = output;
        }
    }
    max_output
}

struct Amplifier {
    software: Intcode,
    state: State,
    input: VecDeque<i64>,
    output: Weak<RefCell<Self>>,
}

impl Amplifier {
    fn new(mut software: Intcode, setting: i64) -> Self {
        let state = software.run(Some(setting));
        Self {
            software,
            state,
            input: VecDeque::new(),
            output: Weak::new(),
        }
    }

    fn set_output(&mut self, output: Weak<RefCell<Self>>) {
        self.output = output;
    }

    fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    fn push_to_output(&self, output: i64) {
        if let Some(amp) = self.output.upgrade() {
            amp.borrow_mut().add_input(output);
        }
    }

    fn run(&mut self) -> Option<i64> {
        let mut output = None;
        loop {
            let state = self.software.run(self.input.pop_front());
            self.state = state;
            match state {
                State::Output(o) => {
                    output = Some(o);
                    self.push_to_output(o);
                }
                State::NeedsInput | State::Halted => break,
            }
        }
        output
    }
}

#[aoc(day7, part2)]
fn part2(input: &Input) -> i64 {
    let mut max_output = 0;
    for permutation in (5..=9).permutations(5) {
        let software = Intcode::new(input.clone());
        let amp_a = Rc::new(RefCell::from(Amplifier::new(
            software.clone(),
            permutation[0],
        )));
        let amp_b = Rc::new(RefCell::from(Amplifier::new(
            software.clone(),
            permutation[1],
        )));
        let amp_c = Rc::new(RefCell::from(Amplifier::new(
            software.clone(),
            permutation[2],
        )));
        let amp_d = Rc::new(RefCell::from(Amplifier::new(
            software.clone(),
            permutation[3],
        )));
        let amp_e = Rc::new(RefCell::from(Amplifier::new(
            software.clone(),
            permutation[4],
        )));
        amp_a.borrow_mut().set_output(Rc::downgrade(&amp_b));
        amp_b.borrow_mut().set_output(Rc::downgrade(&amp_c));
        amp_c.borrow_mut().set_output(Rc::downgrade(&amp_d));
        amp_d.borrow_mut().set_output(Rc::downgrade(&amp_e));
        amp_e.borrow_mut().set_output(Rc::downgrade(&amp_a));

        amp_a.borrow_mut().add_input(0);

        let amps = [
            Rc::clone(&amp_a),
            Rc::clone(&amp_b),
            Rc::clone(&amp_c),
            Rc::clone(&amp_d),
            Rc::clone(&amp_e),
        ];

        while !amps
            .iter()
            .all(|a| matches!(a.borrow().state, State::Halted))
        {
            for amp in &amps {
                amp.borrow_mut().run();
            }
        }

        if let Some(output) = amp_a.borrow().input.front() {
            if *output > max_output {
                max_output = *output;
            }
        };
    }
    max_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for (signal,program) in [
            (43210, "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            (54321, "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            (65210, "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"),
        ] {
            eprintln!("Program: {program}, signal: {signal}");
            assert_eq!(part1(&parse(program)), signal);
        }
    }

    #[test]
    fn part2_example() {
        for (signal,program) in [
            (139629729, "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            (18216, "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
        ] {
            eprintln!("Program: {program}, signal: {signal}");
            assert_eq!(part2(&parse(program)), signal);
        }
    }
}
