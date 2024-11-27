use aoc_runner_derive::{aoc, aoc_generator};

type Spec = (u32, u32, u32);
type Input = Vec<Spec>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut specs = l.split_whitespace();
            let h = specs.next().unwrap().parse().unwrap();
            let w = specs.next().unwrap().parse().unwrap();
            let l = specs.next().unwrap().parse().unwrap();
            (h, w, l)
        })
        .collect()
}

fn valid_triangles(input: &Input) -> usize {
    input
        .iter()
        .filter(|(h, w, l)| h + w > *l && h + l > *w && w + l > *h)
        .count()
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> usize {
    valid_triangles(input)
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> usize {
    let transposed_triangles = input
        .chunks(3)
        .flat_map(|chunk| {
            let tri0 = (chunk[0].0, chunk[1].0, chunk[2].0);
            let tri1 = (chunk[0].1, chunk[1].1, chunk[2].1);
            let tri2 = (chunk[0].2, chunk[1].2, chunk[2].2);
            [tri0, tri1, tri2]
        })
        .collect();
    valid_triangles(&transposed_triangles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("5 10 25")), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "101 301 501
                 102 302 502
                 103 303 503
                 201 401 601
                 202 402 602
                 203 403 603"
            )),
            6
        );
    }
}
