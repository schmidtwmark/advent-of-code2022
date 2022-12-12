#![feature(hash_set_entry)]
use aoc::Solver;
use itertools::Itertools;
use log::{debug, info};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;

struct Graph<V, E> {
    edges: HashMap<V, HashSet<(E, V)>>,
}

impl<V, E> Graph<V, E>
where
    V: Eq + std::hash::Hash + Clone + Copy + Debug,
    E: Eq + std::hash::Hash + Clone + Debug,
{
    fn new() -> Graph<V, E> {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: V, to: V, edge: E) {
        let from_entry = self.edges.entry(from);
        from_entry.or_default().insert((edge, to));
    }

    fn debug(&self) {
        for (vertex, edges) in &self.edges {
            debug!("{:?}: {:?}", vertex, edges);
        }
    }

    fn bfs(&self, start: V, end: V) -> Option<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((vertex, depth)) = queue.pop_front() {
            if visited.contains(&vertex) {
                continue;
            }
            visited.insert(vertex);

            if vertex == end {
                return Some(depth);
            }

            if let Some(edges) = self.edges.get(&vertex) {
                for (_, neighbor) in edges {
                    queue.push_back((*neighbor, depth + 1));
                }
            }
        }
        info!("Never found end!");
        None
    }
}

fn in_range(a: &char, b: &char) -> bool {
    let v = [a, b];
    let (a, b) = v
        .iter()
        .map(|c| match c {
            'S' => 'a' as usize,
            'E' => 'z' as usize,
            _ => **c as usize,
        })
        .collect_tuple()
        .unwrap();
    if b >= a {
        b - a <= 1
    } else {
        true
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let grid = aoc::Grid::<char>::from_lines(lines, &|c| c);

        let mut graph = Graph::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for i in 0..grid.width {
            for j in 0..grid.height {
                let item = grid.at((i, j));
                if *item == 'S' {
                    start = (i, j);
                } else if *item == 'E' {
                    end = (i, j);
                }

                let neighbors = grid.cardinal_neighbor_positions((i, j));

                for neighbor in neighbors {
                    let neighbor_item = grid.at(neighbor);
                    if in_range(item, neighbor_item) {
                        graph.add_edge((i, j), neighbor, ());
                    }

                    if in_range(neighbor_item, item) {
                        graph.add_edge(neighbor, (i, j), ());
                    }
                }
            }
        }
        info!("Searching from {:?} to {:?}", start, end);
        graph.debug();

        graph.bfs(start, end).unwrap()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let grid = aoc::Grid::<char>::from_lines(lines, &|c| c);

        let mut graph = Graph::new();
        let mut starts = vec![];
        let mut end = (0, 0);

        for i in 0..grid.width {
            for j in 0..grid.height {
                let item = grid.at((i, j));
                if *item == 'S' || *item == 'a' {
                    starts.push((i, j));
                } else if *item == 'E' {
                    end = (i, j);
                }

                let neighbors = grid.cardinal_neighbor_positions((i, j));

                for neighbor in neighbors {
                    let neighbor_item = grid.at(neighbor);
                    if in_range(item, neighbor_item) {
                        graph.add_edge((i, j), neighbor, ());
                    }

                    if in_range(neighbor_item, item) {
                        graph.add_edge(neighbor, (i, j), ());
                    }
                }
            }
        }
        graph.debug();
        starts
            .into_iter()
            .filter_map(|start| {
                info!("Searching from {:?} to {:?}", start, end);

                graph.bfs(start, end)
            })
            .min()
            .unwrap()
    }
}

fn main() {
    let sample = include_str!("../../samples/12.txt");
    let input = include_str!("../../inputs/12.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 31),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 29),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
