use aoc_runner_derive::{aoc, aoc_generator};

type Point = (i32, i32);

#[derive(Debug)]
struct SensorBeaconPair {
    sensor: Point,
    beacon: Point,
}

struct Input {
    pairs: Vec<SensorBeaconPair>,
    line_to_check: i32,
}

#[aoc_generator(day15)]
fn parse(input: &str) -> Input {
    let (line_to_check_str, pairs_str) = input.split_once("\n\n").unwrap();
    let line_to_check = line_to_check_str.parse().unwrap();
    let pairs = pairs_str
        .lines()
        .map(|l| {
            let (s_str, b_str) = l.trim().split_once(": ").unwrap();
            let parse_point = |str: &str| {
                let (x_str, y_str) = str.split_once(", ").unwrap();
                let x = x_str.trim_start_matches("x=").parse::<i32>().unwrap();
                let y = y_str.trim_start_matches("y=").parse::<i32>().unwrap();
                (x, y)
            };

            let sensor = parse_point(s_str.trim_start_matches("Sensor at "));
            let beacon = parse_point(b_str.trim_start_matches("closest beacon is at "));

            SensorBeaconPair { sensor, beacon }
        })
        .collect();

    Input {
        pairs,
        line_to_check,
    }
}

fn distance(a: Point, b: Point) -> i32 {
    u2i(a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
}

fn u2i(n: u32) -> i32 {
    n.try_into()
        .expect("we aren't dealing with such big numbers")
}

#[aoc(day15, part1)]
fn part1(input: &Input) -> i32 {
    let ranges = input
        .pairs
        .iter()
        // figure out the ranges for each pair
        .map(|pair| {
            let dist = distance(pair.sensor, pair.beacon);
            // line_dist = sensor y `absdiff` line_to_check
            let line_dist = u2i(pair.sensor.1.abs_diff(input.line_to_check));
            // edge_dist = dist - line_dist
            let edge_dist = u2i(dist.abs_diff(line_dist));
            // range = (x - edge_dist...x + edge_dist)
            (pair.sensor.0 - edge_dist, pair.sensor.0 + edge_dist)
        })
        // then fold into ranges
        .fold(
            vec![],
            |mut ranges: Vec<(i32, i32)>, (range_min, range_max)| {
                // Get all ranges that touch this range
                let mut touching_ranges = vec![];
                let mut i = 0;
                while i < ranges.len() {
                    let is_touching = |(r_min, r_max): (i32, i32)| {
                        (r_min <= range_min && range_min <= r_max)
                            || (r_max >= range_max && range_max >= r_min)
                    };
                    if is_touching(ranges[i]) {
                        touching_ranges.push(ranges.remove(i));
                    } else {
                        // Only increment if we haven't removed anything
                        // This code will be much better once [Vec::extract_if] is stabilized
                        i += 1;
                    }
                }
                // Add the current one as well
                touching_ranges.push((range_min, range_max));
                // Find the min and max of the touching ranges and add back to ranges
                let min = touching_ranges
                    .iter()
                    .map(|(min, _)| min)
                    .min()
                    .expect("there's at least one element in this vec");
                let max = touching_ranges
                    .iter()
                    .map(|(_, max)| max)
                    .max()
                    .expect("there's at least one element in this vec");
                ranges.push((*min, *max));
                ranges
            },
        );

    let positions: i32 = ranges
        .iter()
        // for each ramge find distance (max - min)
        .map(|(min, max)| max - min + 1)
        // and sum
        .sum();

    let mut beacons_on_line: Vec<_> = input
        .pairs
        .iter()
        .map(|p| p.beacon)
        .filter(|(_, y)| *y == input.line_to_check)
        .collect();
    beacons_on_line.dedup();

    let beacons_on_line_count: i32 = beacons_on_line
        .len()
        .try_into()
        .expect("probably small enaough to fit into i32");

    positions - beacons_on_line_count
}

#[aoc(day15, part2)]
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
                "10

                 Sensor at x=2, y=18: closest beacon is at x=-2, y=15
                 Sensor at x=9, y=16: closest beacon is at x=10, y=16
                 Sensor at x=13, y=2: closest beacon is at x=15, y=3
                 Sensor at x=12, y=14: closest beacon is at x=10, y=16
                 Sensor at x=10, y=20: closest beacon is at x=10, y=16
                 Sensor at x=14, y=17: closest beacon is at x=10, y=16
                 Sensor at x=8, y=7: closest beacon is at x=2, y=10
                 Sensor at x=2, y=0: closest beacon is at x=2, y=10
                 Sensor at x=0, y=11: closest beacon is at x=2, y=10
                 Sensor at x=20, y=14: closest beacon is at x=25, y=17
                 Sensor at x=17, y=20: closest beacon is at x=21, y=22
                 Sensor at x=16, y=7: closest beacon is at x=15, y=3
                 Sensor at x=14, y=3: closest beacon is at x=15, y=3
                 Sensor at x=20, y=1: closest beacon is at x=15, y=3"
            )),
            26
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    }
}
