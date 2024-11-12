use std::collections::{HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct Replacement {
    from: String,
    to: String,
}

struct Machine {
    replacements: Vec<Replacement>,
    molecule: String,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Machine {
    let (rs, m) = input.split_once("\n\n").unwrap();

    let replacements = rs
        .lines()
        .map(|l| {
            let (f, t) = l.trim().split_once(" => ").unwrap();
            Replacement {
                from: f.to_string(),
                to: t.to_string(),
            }
        })
        .collect();

    Machine {
        replacements,
        molecule: m.trim().to_string(),
    }
}

#[aoc(day19, part1, Vec)]
fn part1_vec(m: &Machine) -> usize {
    let mut molecules = Vec::with_capacity(m.replacements.len());

    for r in &m.replacements {
        // find all sections of molecule that can be replaced
        let replace_points: Vec<_> = m.molecule.match_indices(&r.from).map(|(i, _)| i).collect();

        // make each replacement and add to cache if it doesn't already exist
        for rp in replace_points {
            let mut new_molecule = m.molecule.clone();
            new_molecule.replace_range(rp..rp + r.from.len(), &r.to);

            if !molecules.contains(&new_molecule) {
                molecules.push(new_molecule);
            }
        }
    }

    // return number of items in cache
    molecules.len()
}

#[aoc(day19, part1, HashSet)]
fn part1_hash_set(m: &Machine) -> usize {
    let mut molecules = HashSet::with_capacity(m.replacements.len());

    for r in &m.replacements {
        // find all sections of molecule that can be replaced
        let replace_points: Vec<_> = m.molecule.match_indices(&r.from).map(|(i, _)| i).collect();

        // make each replacement and add to cache if it doesn't already exist
        for rp in replace_points {
            let mut new_molecule = m.molecule.clone();
            new_molecule.replace_range(rp..rp + r.from.len(), &r.to);

            molecules.insert(new_molecule);
        }
    }

    // return number of items in cache
    molecules.len()
}

#[derive(Default, Debug)]
struct TrieNode<'r> {
    children: HashMap<char, TrieNode<'r>>,
    // Replacements applicable to this point in the trie
    replacements: Vec<&'r Replacement>,
}

struct Trie<'r> {
    root: TrieNode<'r>,
    machine: &'r Machine,
}

impl<'r> Trie<'r> {
    fn new(machine: &'r Machine) -> Trie<'r> {
        Self {
            machine,
            root: TrieNode::default(),
        }
    }

    /// Insert a string into the trie, and return all possible replacements for the string
    fn get_replacements(&mut self, s: &str) -> Vec<(&Replacement, usize)> {
        let mut node = &mut self.root;
        let mut tracking_str = String::new();
        let mut rs = vec![];

        for (i, c) in s.chars().enumerate() {
            tracking_str.push(c);
            node = node.children.entry(c).or_insert_with(|| {
                let replacements = self
                    .machine
                    .replacements
                    .iter()
                    .filter(|r| tracking_str.ends_with(&r.from))
                    .collect();
                TrieNode {
                    replacements,
                    ..Default::default()
                }
            });

            rs.extend(node.replacements.iter().map(|r| (*r, i)));
        }

        rs
    }
}

#[aoc(day19, part2)]
fn part2(m: &Machine) -> usize {
    let init_str = String::from("e");
    let mut paths = VecDeque::from([vec![init_str.clone()]]);
    let mut visited = HashSet::from([init_str.clone()]);
    let mut trie = Trie::new(m);
    let _ = trie.get_replacements(&init_str);

    // for each molecule path in queue
    while let Some(mp) = paths.pop_front() {
        // --- DEBUG
        //eprintln!("{}", mp.join(" -> "));
        // --- DEBUG

        let latest_m = mp.last().unwrap();

        // if last molecule equals target molecule, return path
        if *latest_m == m.molecule {
            //dbg!(&mp);
            // return len - 1 as the first step doesn't count
            return mp.len() - 1;
        }

        // DEBUG TRIE
        //dbg!(&trie.root);
        // DEBUG TRIE

        // find all replacements applicable to last molecule and make them
        let replace_points: Vec<_> = trie.get_replacements(latest_m);

        // push each one into queue
        for (r, i) in replace_points {
            let mut new_path = mp.clone();
            let mut new_molecule =
                String::with_capacity(latest_m.len() - r.from.len() + r.to.len());
            new_molecule += &latest_m[..i];
            new_molecule += &r.to;
            if i + r.from.len() < latest_m.len() {
                new_molecule += &latest_m[i + r.from.len()..];
            }

            // If new molecule has already been seen, skip
            if visited.contains(&new_molecule) {
                continue;
            }

            visited.insert(new_molecule.clone());
            new_path.push(new_molecule);
            paths.push_back(new_path);
        }
    }
    0
}

/// Start from the solution and work backwards to "e" to speed things up
//#[aoc(day19, part2, backwards)]
//fn part2_backwards(m: &Machine) -> usize {
//    let mut paths = VecDeque::from([vec![m.molecule.clone()]]);
//    // for each molecule path in queue
//    while let Some(mp) = paths.pop_front() {
//        // --- DEBUG
//        //eprintln!("{}", mp.join(" <- "));
//        // --- DEBUG
//
//        let latest_m = mp.last().unwrap();
//
//        // if last molecule equals "e", return path
//        if *latest_m == "e" {
//            //dbg!(&mp);
//            // return len - 1 as the first step doesn't count
//            return mp.len() - 1;
//        }
//
//        // find all replacements applicable to last molecule and make them
//        let replace_points: Vec<_> = m
//            .replacements
//            .iter()
//            .flat_map(|r| {
//                latest_m
//                    .match_indices(&r.to.clone())
//                    .map(|(i, _)| {
//                        let r_ = r.clone();
//                        (r_, i)
//                    })
//                    .collect::<Vec<_>>()
//            })
//            .collect();
//        // push each one into queue
//        for (r, i) in replace_points {
//            //eprintln!("{0} <= {1}: {i}", r.to, r.from);
//            //eprintln!("{latest_m}");
//            let mut new_path = mp.clone();
//            let mut new_molecule = latest_m.clone().to_string();
//
//            new_molecule.replace_range(i..i + r.to.len(), &r.from);
//            new_path.push(new_molecule);
//            paths.push_back(new_path);
//        }
//    }
//    0
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        for f in [part1_vec, part1_hash_set] {
            assert_eq!(
                f(&parse(
                    "H => HO
                     H => OH
                     O => HH

                     HOH"
                )),
                4
            );
            assert_eq!(
                f(&parse(
                    "H => HO
                     H => OH
                     O => HH

                     HOHOHO"
                )),
                7
            );
        }
    }

    #[test]
    fn part2_example() {
        for f in [part2] {
            assert_eq!(
                f(&parse(
                    "e => H
                     e => O
                     H => HO
                     H => OH
                     O => HH

                     HOH"
                )),
                3
            );
            assert_eq!(
                f(&parse(
                    "e => H
                     e => O
                     H => HO
                     H => OH
                     O => HH

                     HOHOHO"
                )),
                6
            );
        }
    }
}
