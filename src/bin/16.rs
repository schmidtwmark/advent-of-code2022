#![feature(iter_collect_into)]

use aoc::{Graph, Solver};
use bimap::BiMap;

use itertools::Itertools;
use log::{debug, info};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn parse_lines<'a>(
    lines: &'a [&str],
) -> (
    aoc::Graph<usize, ()>,
    BiMap<&'a str, usize>,
    HashMap<usize, usize>,
) {
    let mut graph = aoc::Graph::new();
    let mut edges = HashMap::new();
    let mut flow_rates = HashMap::new();
    let mut name_map = BiMap::new();
    for (idx, line) in lines.iter().enumerate() {
        let (head, mut tail) = line.trim().split_once(" valve").unwrap();

        let mut splat = head.split(' ');
        let name = splat.nth(1).unwrap();
        let digit = splat.nth(2).unwrap().split_once('=').unwrap().1;

        let flow_rate = digit[..digit.len() - 1].parse().unwrap();

        name_map.insert(name, idx);
        flow_rates.insert(idx, flow_rate);

        if tail.starts_with("s ") {
            tail = &tail[2..];
        }
        // Split on comma, add to edges
        edges.insert(idx, tail.trim().split(", ").collect_vec());
    }

    for (from, to_vec) in edges {
        for to in to_vec {
            graph.add_edge(from, *name_map.get_by_left(to).unwrap(), ());
        }
    }

    (graph, name_map, flow_rates)
}

#[derive(Debug, Clone)]
struct Branch {
    open_valves: HashSet<usize>,
    pressure_released: usize,
    current_node: usize,
    elephant_node: usize,
    should_continue: bool,
    visited_count: HashMap<usize, usize>, // For each visited node, how many times has it been visited?
}

impl std::fmt::Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(Current: {}, Elephant: {}, Pressure: {}, Open: {:?})",
            self.current_node, self.elephant_node, self.pressure_released, self.open_valves
        )
    }
}

impl Branch {
    fn new(start: usize) -> Branch {
        Branch {
            open_valves: HashSet::new(),
            pressure_released: 0,
            current_node: start,
            elephant_node: start,
            should_continue: true,
            visited_count: HashMap::new(),
        }
    }

    fn from(other: &Branch) -> Branch {
        Branch {
            open_valves: other.open_valves.clone(),
            pressure_released: other.pressure_released,
            current_node: other.current_node,
            elephant_node: other.elephant_node,
            should_continue: other.should_continue,
            visited_count: other.visited_count.clone(),
        }
    }

    fn get_pressure(&self, flow_rates: &HashMap<usize, usize>) -> usize {
        self.open_valves.iter().map(|valve| flow_rates[valve]).sum()
    }

    fn step2(
        &mut self,
        graph: &Graph<usize, ()>,
        flow_rates: &HashMap<usize, usize>,
    ) -> impl Iterator<Item = Branch> {
        let mut new_branches = vec![];
        self.pressure_released += self.get_pressure(flow_rates);
        self.visited_count
            .entry(self.current_node)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        self.visited_count
            .entry(self.elephant_node)
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

            // Open self valve, move elephant or elephant stays put
            if can_open_self {
                for neighbor in elephant_neighbors
                    .iter()
                    .map(|(_, n)| n)
                    .chain(std::iter::once(&self.elephant_node))
                {
                    if let Some(neighbor_visits) = self.visited_count.get(neighbor) && *neighbor_visits > graph.get(neighbor).unwrap().len() {
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                    continue;
                }
                    let mut new_branch = Branch::from(self);
                    new_branch.open_valves.insert(self.current_node);
                    new_branch.elephant_node = *neighbor;
                    new_branches.push(new_branch);
                }
            }
            // Open elephant valve, move self or stay stationary
            if can_open_elephant {
                for neighbor in neighbors
                    .iter()
                    .map(|(_, n)| n)
                    .chain(std::iter::once(&self.current_node))
                {
                    if let Some(neighbor_visits) = self.visited_count.get(neighbor) && *neighbor_visits > graph.get(neighbor).unwrap().len() {
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                    continue;
                }
                    let mut new_branch = Branch::from(self);
                    new_branch.current_node = *neighbor;
                    new_branch.open_valves.insert(self.elephant_node);
                    new_branches.push(new_branch);
                }
            }
            // Open both valves, neither moves
            if self.current_node != self.elephant_node && can_open_elephant && can_open_self {
                let mut new_branch = Branch::from(self);
                new_branch.open_valves.insert(self.current_node);
                new_branch.open_valves.insert(self.elephant_node);
                new_branches.push(new_branch);
            }

            let mut new_locations = HashSet::new();
            // Move both or stay stationary
            neighbors
                .iter().map(|(_, n)| n)
                .cartesian_product(elephant_neighbors.iter().map(|(_, n)| n))
                .for_each(|(my_neighbor, elephant_neighbor)| {
                if let Some(neighbor_visits) = self.visited_count.get(my_neighbor) && *neighbor_visits > graph.get(my_neighbor).unwrap().len() * 2{
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                } else if let Some(neighbor_visits) = self.visited_count.get(elephant_neighbor) && *neighbor_visits > graph.get(elephant_neighbor).unwrap().len() * 2{
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                } else if !new_locations.contains(&(my_neighbor, elephant_neighbor)) && !new_locations.contains(&(elephant_neighbor, my_neighbor)) {
                        let mut new_branch = Branch::from(self);
                        new_branch.current_node = *my_neighbor;
                        new_branch.elephant_node = *elephant_neighbor;

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
        graph: &Graph<usize, ()>,
        flow_rates: &HashMap<usize, usize>,
    ) -> impl Iterator<Item = Branch> {
        let mut new_branches = vec![];
        self.pressure_released += self.get_pressure(flow_rates);
        self.visited_count
            .entry(self.current_node)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        if self.should_continue {
            let mut new_branch = Branch::from(self);
            new_branch.should_continue = false;
            new_branches.push(new_branch);
            if !self.open_valves.contains(&self.current_node) && flow_rates[&self.current_node] > 0
            {
                let mut new_branch = Branch::from(self);
                new_branch.open_valves.insert(self.current_node);
                new_branches.push(new_branch);
            }

            let neighbors = graph.get(&self.current_node).unwrap();
            for (_, neighbor) in neighbors {
                if let Some(neighbor_visits) = self.visited_count.get(neighbor) && *neighbor_visits > graph.get(neighbor).unwrap().len() {
                    // If we've already visited this node more times than it has neighbors, we probably shouldn't go back
                    continue;
                }
                let mut new_branch = Branch::from(self);
                new_branch.current_node = *neighbor;
                new_branches.push(new_branch);
            }
        } else {
            new_branches.push(self.clone()); // Do nothing on this branch
        }

        new_branches.into_iter()
    }

    fn score(
        &self,
        all_valves: &HashSet<usize>,
        distances: &HashMap<usize, HashMap<usize, usize>>,
    ) -> i64 {
        let unvisited_valves = all_valves.difference(&self.open_valves);
        unvisited_valves
            .map(|v| distances[&self.current_node][v] as i64)
            .sum::<i64>()
            - self.pressure_released as i64
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (graph, name_map, flow_rates) = parse_lines(lines);
        graph.debug_connections();

        let mut branches = vec![Branch::new(*name_map.get_by_left("AA").unwrap())];

        for minute in 1..=30 {
            let mut new_branches = vec![];
            for branch in branches.iter_mut() {
                new_branches.extend(branch.step(&graph, &flow_rates));
            }

            if new_branches.len() > 100000 {
                let max = new_branches
                    .iter()
                    .max_by(|a, b| a.pressure_released.cmp(&b.pressure_released))
                    .unwrap();
                let min = new_branches
                    .iter()
                    .min_by(|a, b| a.pressure_released.cmp(&b.pressure_released))
                    .unwrap();
                let mean = (max.pressure_released + min.pressure_released) / 2;
                if max.pressure_released == mean || min.pressure_released == mean {
                    new_branches = vec![max.clone()];
                }

                new_branches = new_branches
                    .into_iter()
                    .filter(|b| b.pressure_released >= mean)
                    .collect_vec();
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
        }

        branches
            .into_iter()
            .map(|b| b.pressure_released)
            .max()
            .unwrap()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (graph, name_map, flow_rates) = parse_lines(lines);
        graph.debug_connections();

        let all_valves: HashSet<usize> = graph
            .all_vertices()
            .copied()
            .filter(|v| flow_rates[v] > 0)
            .collect();
        let distance: HashMap<usize, HashMap<usize, usize>> = graph
            .all_vertices()
            .map(|v| {
                (
                    *v,
                    graph
                        .all_distances(v)
                        .into_iter()
                        .map(|(target, distance)| (*target, flow_rates[target] * distance))
                        .collect(),
                )
            })
            .collect();

        let mut branches = vec![Branch::new(*name_map.get_by_left("AA").unwrap())];

        for minute in 1..=26 {
            let mut new_branches = vec![];
            for branch in branches.iter_mut() {
                new_branches.extend(branch.step2(&graph, &flow_rates));
            }

            while new_branches.len() > 100000 {
                // let scores = new_branches.into_iter().map(|b| {
                //     let score = b.score(&all_valves, &distance);
                //     (b, score)
                // });
                // let min = scores.clone().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
                // let max = scores.clone().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
                // let mean = (min.1 + max.1) / 2;
                // debug!("Min: {}, Max: {}, Mean: {}", min.1, max.1, mean);
                //                 if max.1 == mean || min.1 == mean {
                //     new_branches = vec![max.0.clone()];
                //     break;
                // }
                // new_branches = scores
                //     .filter_map(|(b, s)| if s <= mean { Some(b) } else { None })
                //     .collect_vec();
                let max = new_branches
                    .iter()
                    .max_by(|a, b| a.pressure_released.cmp(&b.pressure_released))
                    .unwrap();
                let min = new_branches
                    .iter()
                    .min_by(|a, b| a.pressure_released.cmp(&b.pressure_released))
                    .unwrap();
                let mean = (max.pressure_released + min.pressure_released) / 2;
                if max.pressure_released == mean || min.pressure_released == mean {
                    new_branches = vec![max.clone()];
                }

                new_branches = new_branches
                    .into_iter()
                    .filter(|b| b.pressure_released >= mean)
                    .collect_vec();
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
        aoc::Input::new_final(input), // 2811 is the right answer
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
