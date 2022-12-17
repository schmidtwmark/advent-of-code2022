#![feature(iter_collect_into)]

use aoc::{Graph, Solver};
use bimap::BiMap;

use itertools::Itertools;
use log::debug;
use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn parse_lines<'a>(
    lines: &'a [&str],
) -> (
    aoc::Graph<Vertex, ()>,
    BiMap<&'a str, Vertex>,
    HashMap<Vertex, FlowRate>,
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
    open_valves: HashSet<Vertex>,
    pressure_released: usize,
    my_node: Vertex,
    elephant_node: Vertex,
    my_arrival: Option<(Vertex, Minute)>,
    elephant_arrival: Option<(Vertex, Minute)>,
}

#[derive(Debug, Clone)]
enum Movement {
    Continue,
    OpenValve(Vertex),
    Move(Vertex, Minute),
}

impl Branch {
    fn new(start: usize) -> Branch {
        Branch {
            open_valves: HashSet::new(),
            pressure_released: 0,
            my_node: start,
            elephant_node: start,
            my_arrival: None,
            elephant_arrival: None,
        }
    }

    fn from(other: &Branch) -> Branch {
        Branch {
            open_valves: other.open_valves.clone(),
            pressure_released: other.pressure_released,
            my_node: other.my_node,
            elephant_node: other.elephant_node,
            my_arrival: other.my_arrival,
            elephant_arrival: other.elephant_arrival,
        }
    }

    fn get_pressure(&self, flow_rates: &HashMap<usize, usize>) -> usize {
        self.open_valves.iter().map(|valve| flow_rates[valve]).sum()
    }

    fn calc_movement(
        arrival: &Option<(Vertex, Minute)>,
        minute: Minute,
        node: Vertex,
        weighted_graph: &Graph<Vertex, Distance>,
        available_valves: &HashSet<Vertex>,
    ) -> Vec<Movement> {
        match arrival {
            None => {
                let mut moves = vec![Movement::Continue];
                for available_valve in available_valves {
                    if *available_valve == node {
                        continue;
                    }
                    let distances = weighted_graph.get(&node).unwrap();
                    let distance = distances.get(available_valve).unwrap();

                    moves.push(Movement::Move(*available_valve, minute + distance));
                }
                moves
            }
            Some((new_node, arrival)) => {
                if *arrival == minute {
                    vec![Movement::OpenValve(*new_node)]
                } else {
                    vec![Movement::Continue]
                }
            }
        }
    }
    fn step(
        &mut self,
        minute: usize,
        weighted_graph: &Graph<Vertex, Distance>,
        flow_rates: &HashMap<Vertex, FlowRate>,
    ) -> impl Iterator<Item = Branch> + '_ {
        self.pressure_released += self.get_pressure(flow_rates);

        let available_valves: HashSet<Vertex> = weighted_graph
            .all_vertices()
            .copied()
            .filter(|v| !self.open_valves.contains(v))
            .collect();

        let my_moves = Self::calc_movement(
            &self.my_arrival,
            minute,
            self.my_node,
            weighted_graph,
            &available_valves,
        );

        my_moves.into_iter().map(move |my_move| {
            let mut new_branch = Branch::from(self);
            match my_move {
                Movement::Continue => {}
                Movement::OpenValve(valve) => {
                    new_branch.open_valves.insert(valve);
                    new_branch.my_arrival = None
                }
                Movement::Move(node, arrival) => {
                    new_branch.my_node = node;
                    new_branch.my_arrival = Some((node, arrival));
                }
            }
            new_branch
        })
    }

    fn step2(
        &mut self,
        minute: usize,
        weighted_graph: &Graph<Vertex, Distance>,
        flow_rates: &HashMap<Vertex, FlowRate>,
    ) -> impl Iterator<Item = Branch> + '_ {
        self.pressure_released += self.get_pressure(flow_rates);

        let available_valves: HashSet<Vertex> = weighted_graph
            .all_vertices()
            .copied()
            .filter(|v| !self.open_valves.contains(v))
            .collect();

        let my_moves = Self::calc_movement(
            &self.my_arrival,
            minute,
            self.my_node,
            weighted_graph,
            &available_valves,
        );
        let elephant_moves = Self::calc_movement(
            &self.elephant_arrival,
            minute,
            self.elephant_node,
            weighted_graph,
            &available_valves,
        );

        my_moves
            .into_iter()
            .cartesian_product(elephant_moves.into_iter())
            .map(move |(my_move, elephant_move)| {
                let mut new_branch = Branch::from(self);
                match my_move {
                    Movement::Continue => {}
                    Movement::OpenValve(valve) => {
                        new_branch.open_valves.insert(valve);
                        new_branch.my_arrival = None
                    }
                    Movement::Move(node, arrival) => {
                        new_branch.my_node = node;
                        new_branch.my_arrival = Some((node, arrival));
                    }
                }
                match elephant_move {
                    Movement::Continue => {}
                    Movement::OpenValve(valve) => {
                        new_branch.open_valves.insert(valve);
                        new_branch.elephant_arrival = None;
                    }
                    Movement::Move(node, arrival) => {
                        new_branch.elephant_node = node;
                        new_branch.elephant_arrival = Some((node, arrival));
                    }
                }
                new_branch
            })
    }
}

type Vertex = usize;
type FlowRate = usize;
type Distance = usize;
type Minute = usize;
struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (graph, name_map, flow_rates) = parse_lines(lines);

        let start = *name_map.get_by_left("AA").unwrap();
        let mut weighted_graph: Graph<Vertex, Distance> = aoc::Graph::new();
        for (vertex, rate) in flow_rates.iter() {
            if *rate != 0 || *vertex == start {
                let distances = graph.all_distances(vertex);

                for (other, distance) in distances {
                    if other != vertex && (flow_rates[other] != 0 || *other == start) {
                        weighted_graph.add_edge(*vertex, *other, distance);
                    }
                }
            }
        }

        // Debug print
        for (vertex, edges) in &weighted_graph.edges {
            debug!(
                "{}: {:?}",
                name_map.get_by_right(vertex).unwrap(),
                edges
                    .iter()
                    .map(|(v, d)| (name_map.get_by_right(v).unwrap(), d))
                    .collect_vec()
            );
        }

        let mut branches = vec![Branch::new(start)];

        for minute in 1..=30 {
            let mut new_branches = vec![];
            for branch in &mut branches {
                new_branches.extend(branch.step(minute, &weighted_graph, &flow_rates));
            }
            debug!("Minute {}: Num Branches: {}", minute, new_branches.len());

            if new_branches.len() > 500000 {
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
        }

        branches
            .into_iter()
            .map(|b| b.pressure_released)
            .max()
            .unwrap()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (graph, name_map, flow_rates) = parse_lines(lines);

        let start = *name_map.get_by_left("AA").unwrap();
        let mut weighted_graph: Graph<Vertex, Distance> = aoc::Graph::new();
        for (vertex, rate) in flow_rates.iter() {
            if *rate != 0 || *vertex == start {
                let distances = graph.all_distances(vertex);

                for (other, distance) in distances {
                    if other != vertex && (flow_rates[other] != 0 || *other == start) {
                        weighted_graph.add_edge(*vertex, *other, distance);
                    }
                }
            }
        }

        // Debug print
        for (vertex, edges) in &weighted_graph.edges {
            debug!(
                "{}: {:?}",
                name_map.get_by_right(vertex).unwrap(),
                edges
                    .iter()
                    .map(|(v, d)| (name_map.get_by_right(v).unwrap(), d))
                    .collect_vec()
            );
        }

        let mut branches = vec![Branch::new(start)];

        for minute in 1..=26 {
            let mut new_branches = vec![];
            for branch in &mut branches {
                new_branches.extend(branch.step2(minute, &weighted_graph, &flow_rates));
            }
            debug!("Minute {}: Num Branches: {}", minute, new_branches.len());

            if new_branches.len() > 5000000 {
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
