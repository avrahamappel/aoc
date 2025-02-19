use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(input: &str) -> String {
    input.to_string()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let area = 25 * 6;
    let layer = input
        .as_bytes()
        .chunks(area)
        .min_by_key(|layer| layer.iter().filter(|c| **c == b'0').count())
        .unwrap();
    let ones = layer.iter().filter(|c| **c == b'1').count();
    let twos = layer.iter().filter(|c| **c == b'2').count();
    ones * twos
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    const AREA: usize = 25 * 6;
    let layers = input.as_bytes().chunks(AREA).collect::<Vec<_>>();
    let mut final_layer = [b'2'; AREA];
    for i in 0..AREA {
        for layer in &layers {
            if layer[i] != b'2' {
                final_layer[i] = layer[i];
                break;
            }
        }
    }
    let image = final_layer
        .chunks(25)
        .map(|row| {
            row.iter()
                .map(|c| if *c == b'1' { '█' } else { ' ' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("{image}");
    0
}
