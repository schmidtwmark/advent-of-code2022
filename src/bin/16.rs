#![feature(iter_collect_into)]

use aoc::{Graph, Solver};
use itertools::Itertools;
use log::debug;
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
    should_continue: bool,
    visited_count: HashMap<String, usize>, // For each visited node, how many times has it been visited?
}

impl Branch {
    fn new() -> Branch {
        Branch {
            open_valves: HashSet::new(),
            pressure_released: 0,
            current_node: "AA".to_owned(),
            should_continue: true,
            visited_count: HashMap::new(),
        }
    }

    fn from(other: &Branch) -> Branch {
        Branch {
            open_valves: other.open_valves.clone(),
            pressure_released: other.pressure_released,
            current_node: other.current_node.clone(),
            should_continue: other.should_continue,
            visited_count: other.visited_count.clone(),
        }
    }

    fn get_pressure(&self, flow_rates: &HashMap<String, usize>) -> usize {
        self.open_valves.iter().map(|valve| flow_rates[valve]).sum()
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
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
