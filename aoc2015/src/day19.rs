use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Clone, Debug)]
struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    /// Perform replacement on the given molecule at the given index
    fn replace(&self, molecule: &str, idx: usize) -> String {
        let mut new_molecule =
            String::with_capacity(molecule.len() - self.from.len() + self.to.len());
        new_molecule += &molecule[..idx];
        new_molecule += &self.to;
        if idx + self.from.len() < molecule.len() {
            new_molecule += &molecule[idx + self.from.len()..];
        }
        new_molecule
    }
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

/// Represents a path to creating a molecule from "e"
#[derive(PartialEq, Eq, Debug, Clone)]
struct Path<'t> {
    path: Vec<String>,
    target: &'t str,
}

/// How to sort possible molecule paths
impl Ord for Path<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s_last = self.path.last();
        let o_last = other.path.last();
        let get_pres = |o: Option<&String>| o.map(|s| self.target.contains(s));
        let get_len = |o: Option<&String>| o.map(String::len);

        // Compare by size
        get_len(o_last).cmp(&get_len(s_last)).then_with(||
        // Compare by presence in target
        get_pres(s_last).cmp(&get_pres(o_last)))
    }
}
impl PartialOrd for Path<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part2(m: &Machine) -> usize {
    let init_str = String::from("e");
    let mut paths = BinaryHeap::from([Path {
        path: vec![init_str.clone()],
        target: &m.molecule,
    }]);
    let mut visited = HashSet::from([init_str.clone()]);
    let mut trie = Trie::new(m);
    let _ = trie.get_replacements(&init_str);

    // for each molecule path in queue
    while let Some(mp) = paths.pop() {
        // --- DEBUG
        eprintln!("{}", mp.path.join(" -> "));
        // --- DEBUG

        let latest_m = mp.path.last().unwrap();

        // if last molecule equals target molecule, return path
        if *latest_m == m.molecule {
            //dbg!(&mp);
            // return len - 1 as the first step doesn't count
            return mp.path.len() - 1;
        }

        // DEBUG TRIE
        //dbg!(&trie.root);
        // DEBUG TRIE

        // find all replacements applicable to last molecule and make them
        let replace_points = trie.get_replacements(latest_m);

        // push each one into queue
        for (r, i) in replace_points {
            let mut new_path = mp.clone();
            let new_molecule = r.replace(latest_m, i);

            // If molecule is larger than target, skip
            if new_molecule.len() > m.molecule.len() {
                continue;
            }

            // If new molecule has already been seen, skip
            if visited.contains(&new_molecule) {
                continue;
            }

            visited.insert(new_molecule.clone());
            new_path.path.push(new_molecule);
            paths.push(new_path);
        }
    }
    0
}

/// Implementation of the A* search algorithm
/// <https://en.wikipedia.org/wiki/A*_search_algorithm>
fn a_star(start: String, goal: &str, mut repl_trie: Trie) -> usize {
    let mut queue = BinaryHeap::from([(0, start.clone())]);
    let mut visited = HashSet::from([start.clone()]);
    let mut g_cost = HashMap::from([(start, 0)]);

    while let Some((f_cost, current)) = queue.pop() {
        eprintln!("{f_cost} - {current}");
        if current == goal {
            return g_cost[&current];
        }

        if current.len() < goal.len() {
            let replacements = repl_trie.get_replacements(&current);
            for (r, i) in replacements {
                let new_molecule = r.replace(&current, i);
                // If molecule is larger than target, skip
                if new_molecule.len() > goal.len() {
                    continue;
                }

                // If new molecule has already been seen, skip
                if visited.contains(&new_molecule) {
                    continue;
                }

                visited.insert(new_molecule.clone());
                g_cost.insert(new_molecule.clone(), g_cost[&current] + 1);
                let h_cost = estimate_cost(&new_molecule, goal);
                let f_cost = g_cost[&new_molecule] + h_cost;
                queue.push((f_cost, new_molecule));
            }
        }
    }
    0 // Indicates failure
}

/// Estimate the cost of transforming the given molecule into the goal molecule
fn estimate_cost(molecule: &str, goal: &str) -> usize {
    let length_diff = if molecule.len() > goal.len() {
        0
    } else {
        goal.len() - molecule.len()
    };
    length_diff
}

#[aoc(day19, part2)]
fn part2_a_star(m: &Machine) -> usize {
    a_star("e".into(), &m.molecule, Trie::new(m))
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
        for f in [part2, part2_a_star] {
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
