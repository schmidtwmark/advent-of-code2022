#![feature(iter_collect_into)]

use aoc::{Graph, Solver};
use itertools::Itertools;
use log::{debug, error, info};
use std::collections::{HashMap, HashSet};

fn parse_lines(lines: &[&str]) -> (aoc::Graph<String, ()>, HashMap<String, usize>) {
    let mut graph = aoc::Graph::new();
    let mut edges = HashMap::new();
    let mut flow_rates = HashMap::new();
    for line in lines {
        let (head, mut tail) = line.trim().split_once(" valve").unwrap();

        let mut splat = head.split(' ');
        let name = splat.nth(1).unwrap();
        let digit = splat.nth(2).unwrap().split_once('=').unwrap().1;

        let flow_rate = digit[..digit.len() - 1].parse().unwrap();

        flow_rates.insert(name.to_owned(), flow_rate);

        if tail.starts_with("s ") {
            tail = &tail[2..];
        }

        // Split on comma, add to edges
        edges.insert(name, tail.trim().split(", ").collect_vec());
    }

    for (from, to_vec) in edges {
        for to in to_vec {
            graph.add_edge(from.to_owned(), to.to_owned(), ());
        }
    }

    (graph, flow_rates)
}

#[derive(Debug, Clone)]
struct Branch {
    open_valves: HashSet<String>,
    pressure_released: usize,
    current_node: String,
    elephant_node: String,
    should_continue: bool,
    visited_count: HashMap<String, usize>, // For each visited node, how many times has it been visited?
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Current: '{}', Elephant: '{}', Pressure: {}, Open: {:?})",
            self.current_node, self.elephant_node, self.pressure_released, self.open_valves
        )
    }
}

impl Branch {
    fn new() -> Branch {
        Branch {
            open_valves: HashSet::new(),
            pressure_released: 0,
            current_node: "AA".to_owned(),
            elephant_node: "AA".to_owned(),
            should_continue: true,
            visited_count: HashMap::new(),
        }
    }

    fn from(other: &Branch) -> Branch {
        Branch {
            open_valves: other.open_valves.clone(),
            pressure_released: other.pressure_released,
            current_node: other.current_node.clone(),
            elephant_node: other.elephant_node.clone(),
            should_continue: other.should_continue,
            visited_count: other.visited_count.clone(),
        }
    }

    fn get_pressure(&self, flow_rates: &HashMap<String, usize>) -> usize {
        self.open_valves.iter().map(|valve| flow_rates[valve]).sum()
    }

    fn step2(
        &mut self,
        graph: &Graph<String, ()>,
        flow_rates: &HashMap<String, usize>,
    ) -> impl Iterator<Item = Branch> {
        let mut new_branches = vec![];
        self.pressure_released += self.get_pressure(flow_rates);
        self.visited_count
            .entry(self.current_node.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        self.visited_count
            .entry(self.elephant_node.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);
        // Possible actions
        // 1. Sit still and never move again (vibes)
        // 2. Open self valve, elephant moves
        // 3. Open elephant valve, self moves
        // 4. Open both valves, both move

        if self.should_continue {
            // Never move again
            let mut new_branch = Branch::from(self);
            new_branch.should_continue = false;
            new_branches.push(new_branch);

            let can_open_self = !self.open_valves.contains(&self.current_node)
                && flow_rates[&self.current_node] > 0;
            let can_open_elephant = !self.open_valves.contains(&self.elephant_node)
                && flow_rates[&self.elephant_node] > 0;

            let neighbors = graph.get(&self.current_node).unwrap();
            let elephant_neighbors = graph.get(&self.elephant_node).unwrap();

            // Open self valve, move elephant
            if can_open_self {
                for (_, neighbor) in elephant_neighbors {
                    if let Some(neighbor_visits) = self.visited_count.get(neighbor) && *neighbor_visits > graph.get(neighbor).unwrap().len() {
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                    continue;
                }
                    let mut new_branch = Branch::from(self);
                    new_branch.open_valves.insert(self.current_node.clone());
                    new_branch.elephant_node = neighbor.clone();
                    new_branches.push(new_branch);
                }
            }
            // Open elephant valve, move self
            if can_open_elephant {
                for (_, neighbor) in neighbors {
                    if let Some(neighbor_visits) = self.visited_count.get(neighbor) && *neighbor_visits > graph.get(neighbor).unwrap().len() {
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                    continue;
                }
                    let mut new_branch = Branch::from(self);
                    new_branch.current_node = neighbor.clone();
                    new_branch.open_valves.insert(self.elephant_node.clone());
                    new_branches.push(new_branch);
                }
            }
            // Open both valves, do not move
            if self.current_node != self.elephant_node && can_open_elephant && can_open_self {
                let mut new_branch = Branch::from(self);
                new_branch.open_valves.insert(self.current_node.clone());
                new_branch.open_valves.insert(self.elephant_node.clone());
                new_branches.push(new_branch);
            }

            let mut new_locations = HashSet::new();
            // Move both
            neighbors
                .iter()
                .cartesian_product(elephant_neighbors.iter())
                .for_each(|((_, my_neighbor), (_, elephant_neighbor))| {
                if let Some(neighbor_visits) = self.visited_count.get(my_neighbor) && *neighbor_visits > graph.get(my_neighbor).unwrap().len() * 2{
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                } else if let Some(neighbor_visits) = self.visited_count.get(elephant_neighbor) && *neighbor_visits > graph.get(elephant_neighbor).unwrap().len() * 2{
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                } else if !new_locations.contains(&(my_neighbor, elephant_neighbor)) && !new_locations.contains(&(elephant_neighbor, my_neighbor)) {
                        let mut new_branch = Branch::from(self);
                        new_branch.current_node = my_neighbor.clone();
                        new_branch.elephant_node = elephant_neighbor.clone();

                        new_branches.push(new_branch);
                        new_locations.insert((my_neighbor, elephant_neighbor));
                }
                });
        } else {
            // Continue doing nothing
            new_branches.push(self.clone());
        }

        new_branches.into_iter()
    }

    fn step(
        &mut self,
        graph: &Graph<String, ()>,
        flow_rates: &HashMap<String, usize>,
    ) -> impl Iterator<Item = Branch> {
        let mut new_branches = vec![];
        self.pressure_released += self.get_pressure(flow_rates);
        self.visited_count
            .entry(self.current_node.clone())
            .and_modify(|e| *e += 1)
            .or_insert(1);

        if self.should_continue {
            let mut new_branch = Branch::from(self);
            new_branch.should_continue = false;
            new_branches.push(new_branch);
            if !self.open_valves.contains(&self.current_node) && flow_rates[&self.current_node] > 0
            {
                let mut new_branch = Branch::from(self);
                new_branch.open_valves.insert(self.current_node.clone());
                new_branches.push(new_branch);
            }

            let neighbors = graph.get(&self.current_node).unwrap();
            for (_, neighbor) in neighbors {
                if let Some(neighbor_visits) = self.visited_count.get(neighbor) && *neighbor_visits > graph.get(neighbor).unwrap().len() {
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                    continue;
                }
                let mut new_branch = Branch::from(self);
                new_branch.current_node = neighbor.clone();
                new_branches.push(new_branch);
            }
        } else {
            new_branches.push(self.clone()); // Do nothing on this branch
        }

        new_branches.into_iter()
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (graph, flow_rates) = parse_lines(lines);
        graph.debug();

        let mut branches = vec![Branch::new()];

        for minute in 1..=30 {
            let mut new_branches = vec![];
            for branch in branches.iter_mut() {
                new_branches.extend(branch.step(&graph, &flow_rates));
            }

            if new_branches.len() > 100000 {
                let mean = new_branches
                    .iter()
                    .map(|b| b.pressure_released)
                    .sum::<usize>()
                    / new_branches.len();
                new_branches = new_branches
                    .into_iter()
                    .filter(|b| b.pressure_released > mean)
                    .collect_vec();
            }
            branches = new_branches;
            info!("Minute {}: Num Branches: {}", minute, branches.len());
        }

        branches
            .into_iter()
            .map(|b| b.pressure_released)
            .max()
            .unwrap()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (graph, flow_rates) = parse_lines(lines);
        graph.debug();

        let mut branches = vec![Branch::new()];

        // On the sample, I expect >= 1 branch with current/elephant position and enabled valves
        let expected_positions = [
            ("II", "DD", 0, vec![]),
            ("JJ", "DD", 0, vec!["DD"]),
            ("JJ", "EE", 20, vec!["DD", "JJ"]),
            ("II", "FF", 61, vec!["DD", "JJ"]),
            ("AA", "GG", 102, vec!["DD", "JJ"]),
            ("BB", "HH", 143, vec!["DD", "JJ"]),
            ("BB", "HH", 184, vec!["DD", "JJ", "BB", "HH"]),
            ("CC", "GG", 260, vec!["DD", "JJ", "BB", "HH"]),
            ("CC", "FF", 336, vec!["DD", "JJ", "BB", "HH", "CC"]),
            ("CC", "EE", 414, vec!["DD", "JJ", "BB", "HH", "CC"]),
            ("CC", "EE", 492, vec!["DD", "JJ", "BB", "HH", "CC", "EE"]),
        ];

        for minute in 1..=26 {
            let mut new_branches = vec![];
            for branch in branches.iter_mut() {
                new_branches.extend(branch.step2(&graph, &flow_rates));
            }
            if let Some(expected) = expected_positions.get(minute - 1) {
                if let Some(actual) = new_branches.iter().find(|b| {
                    b.current_node == expected.0
                        && b.elephant_node == expected.1
                        && b.pressure_released == expected.2
                        && b.open_valves == expected.3.iter().map(|s| s.to_string()).collect()
                }) {
                    debug!(
                        "Minute {}: Found matching branch in new branches for expected:\n{:?}\n{:?}",
                        minute, expected, actual
                    );
                } else {
                    error!(
                        "Minute {}: Failed to find matching branch in new branches for {:?}",
                        minute, expected
                    );
                }
            }

            while new_branches.len() > 500000 {
                let mean = new_branches
                    .iter()
                    .map(|b| b.pressure_released)
                    .sum::<usize>()
                    / new_branches.len();
                let current_size = new_branches.len();
                new_branches = new_branches
                    .into_iter()
                    .filter(|b| b.pressure_released >= mean)
                    .collect_vec();
                if new_branches.len() == current_size {
                    new_branches = new_branches[..new_branches.len() / 2].to_vec();
                }
            }
            branches = new_branches;
            let branch = branches
                .iter()
                .max_by(|a, b| a.pressure_released.cmp(&b.pressure_released))
                .unwrap();

            // let branch = &branches[0];
            info!(
                "Minute {}: Num Branches: {}\nBranch: {}",
                minute,
                branches.len(),
                branch
            );
            if let Some(expected) = expected_positions.get(minute - 1) {
                if let Some(actual) = branches.iter().find(|b| {
                    b.current_node == expected.0
                        && b.elephant_node == expected.1
                        && b.pressure_released == expected.2
                        && b.open_valves == expected.3.iter().map(|s| s.to_string()).collect()
                }) {
                    debug!(
                        "Minute {}: Found matching branch for expected:\n{:?}\n{:?}",
                        minute, expected, actual
                    );
                } else {
                    error!(
                        "Minute {}: Failed to find matching branch for {:?}",
                        minute, expected
                    );
                }
            }
        }

        let max_branch = branches
            .into_iter()
            .max_by(|a, b| a.pressure_released.cmp(&b.pressure_released))
            .unwrap();
        info!("Max Branch: {:?}", max_branch);
        max_branch.pressure_released
    }
}

fn main() {
    let sample = include_str!("../../samples/16.txt");
    let input = include_str!("../../inputs/16.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 1651),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 1707),
        aoc::Input::new_final(input), // 2601 too low
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
