use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use jiff::civil::DateTime;

type GuardId = u32;
type Input = Vec<Event>;

#[derive(Debug)]
enum GuardEvent {
    Begin,
    Sleep,
    Wake,
}

#[derive(Debug)]
struct Event {
    date: DateTime,
    guard_id: GuardId,
    event: GuardEvent,
}

impl Event {
    fn parse(l: &str, mut guard_id: GuardId) -> Option<Self> {
        let (date_str, l) = l.trim_start().trim_start_matches('[').split_once("] ")?;
        let date = date_str.parse().ok()?;
        let mut words_iter = l.split_ascii_whitespace();
        let event = match words_iter.next()? {
            "Guard" => {
                guard_id = words_iter.next()?.trim_start_matches('#').parse().ok()?;
                GuardEvent::Begin
            }
            "wakes" => GuardEvent::Wake,
            _ => GuardEvent::Sleep,
        };

        Some(Self {
            date,
            guard_id,
            event,
        })
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    let mut guard_id = 0;
    let mut evts = vec![];
    for line in input.lines() {
        if let Some(evt) = Event::parse(line, guard_id) {
            guard_id = evt.guard_id;
            evts.push(evt);
        }
    }
    evts
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> u32 {
    let mut guard_sleep_spans: HashMap<_, Vec<(DateTime, DateTime)>> = HashMap::new();
    let mut guard_id = 0;
    let mut sleep_time = DateTime::ZERO;

    for evt in input {
        match evt.event {
            GuardEvent::Begin => {
                guard_id = evt.guard_id;
            }
            GuardEvent::Sleep => {
                sleep_time = evt.date;
            }
            GuardEvent::Wake => {
                let span = (sleep_time, evt.date);
                guard_sleep_spans
                    .entry(guard_id)
                    .and_modify(|spans| spans.push(span))
                    .or_insert(vec![span]);
            }
        }
    }

    let sleepiest_guard = guard_sleep_spans
        .iter()
        .max_by_key(|kv| {
            kv.1.iter().fold(0, |acc, (s, e)| {
                acc + s
                    .until(*e)
                    .expect("Generating a span should work")
                    .get_minutes()
            })
        })
        .expect("There should be at least one guard")
        .0
        .to_owned();

    let mut guard_sleep_minutes = HashMap::new();
    for (start, end) in guard_sleep_spans.get_mut(&sleepiest_guard).unwrap() {
        for m in start.minute()..end.minute() {
            guard_sleep_minutes
                .entry(m)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    //dbg!(&guard_sleep_minutes);
    let most_sleepy_minute = guard_sleep_minutes
        .iter()
        .max_by_key(|kv| kv.1)
        .expect("There should be one minute that is the most sleepy")
        .0;

    //dbg!(sleepiest_guard, most_sleepy_minute);
    sleepiest_guard * (*most_sleepy_minute as u32)
}

#[aoc(day4, part2)]
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
                "[1518-11-01 00:00] Guard #10 begins shift
                 [1518-11-01 00:05] falls asleep
                 [1518-11-01 00:25] wakes up
                 [1518-11-01 00:30] falls asleep
                 [1518-11-01 00:55] wakes up
                 [1518-11-01 23:58] Guard #99 begins shift
                 [1518-11-02 00:40] falls asleep
                 [1518-11-02 00:50] wakes up
                 [1518-11-03 00:05] Guard #10 begins shift
                 [1518-11-03 00:24] falls asleep
                 [1518-11-03 00:29] wakes up
                 [1518-11-04 00:02] Guard #99 begins shift
                 [1518-11-04 00:36] falls asleep
                 [1518-11-04 00:46] wakes up
                 [1518-11-05 00:03] Guard #99 begins shift
                 [1518-11-05 00:45] falls asleep
                 [1518-11-05 00:55] wakes up"
            )),
            240
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
