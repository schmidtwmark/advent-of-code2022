use aoc::Solver;
use hashbrown::HashMap;
use itertools::Itertools;
use log::{debug, info};
use sscanf::scanf;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl FromStr for Resource {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(Self::Ore),
            "clay" => Ok(Self::Clay),
            "obsidian" => Ok(Self::Obsidian),
            "geode" => Ok(Self::Geode),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    elements: HashMap<Resource, ResourceCounts>,
    id: usize,
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (preamble, elements_str) = s.split_once(": ").unwrap();
        let id = scanf!(preamble, "Blueprint {}", usize).unwrap();
        let elements = elements_str.split(". ").map(|element| {
            let (resource_str, costs_str) = element.split_once(" costs ").unwrap();
            let resource_type =
                Resource::from_str(scanf!(resource_str, "Each {} robot", str).unwrap()).unwrap();
            let costs = costs_str.split(" and ").map(|cost| {
                let cost = cost.trim_end_matches('.');
                let (count, resource) = scanf!(cost, "{} {}", usize, str).unwrap();
                let resource = Resource::from_str(resource).unwrap();
                (resource, count)
            });
            let costs = ResourceCounts::from(costs);
            (resource_type, costs)
        });

        Ok(Self {
            id,
            elements: elements.collect(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Branch {
    resource_counts: ResourceCounts,
    robot_counts: ResourceCounts,
    robot_in_production: Option<Resource>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ResourceCounts {
    resources: [usize; 4],
}

impl ResourceCounts {
    fn new() -> Self {
        Self { resources: [0; 4] }
    }

    fn new_robots() -> Self {
        Self {
            resources: [1, 0, 0, 0],
        }
    }

    fn from(iterator: impl Iterator<Item = (Resource, usize)>) -> Self {
        let mut resources = [0; 4];
        for (resource, count) in iterator {
            resources[resource as usize] = count;
        }
        Self { resources }
    }

    fn _add_resource(&mut self, resource: Resource, count: usize) {
        self.resources[resource as usize] += count;
    }

    fn get_resource(&self, resource: Resource) -> usize {
        self.resources[resource as usize]
    }

    fn add(&mut self, other: &Self) {
        for (resource, count) in other.resources.iter().enumerate() {
            self.resources[resource] += count;
        }
    }

    fn less_than_equal(&self, other: &Self) -> bool {
        self.resources
            .iter()
            .zip(other.resources.iter())
            .all(|(a, b)| a <= b)
    }

    fn subtract(&mut self, other: &Self) {
        for (resource, count) in other.resources.iter().enumerate() {
            self.resources[resource] -= count;
        }
    }
}

impl Branch {
    fn new() -> Self {
        Self {
            resource_counts: ResourceCounts::new(),
            robot_counts: ResourceCounts::new_robots(),
            robot_in_production: None,
        }
    }

    fn simulate(&mut self, blueprint: &Blueprint) -> impl Iterator<Item = Self> {
        let mut new_branches = vec![];

        self.resource_counts.add(&self.robot_counts);

        for (robot, costs) in blueprint.elements.iter() {
            if costs.less_than_equal(&self.resource_counts) {
                let mut new_branch = *self;
                new_branch.robot_in_production = Some(*robot);
                new_branch.resource_counts.subtract(costs);
                new_branches.push(new_branch);
            }
        }

        if new_branches.is_empty() {
            // Do nothing except gain resources
            new_branches.push(*self);
        }

        new_branches.into_iter()
    }
}

impl Blueprint {
    fn simulate(&self, minutes: usize) -> usize {
        let mut minute = 1;

        let mut branches = vec![Branch::new()];

        while minute < minutes {
            branches = branches
                .into_iter()
                .flat_map(|mut branch| branch.simulate(self))
                .collect_vec();

            minute += 1;
        }

        let best_branch = branches
            .into_iter()
            .max_by_key(|b| b.resource_counts.get_resource(Resource::Geode))
            .unwrap();

        info!(
            "For blueprint #{}, the best branch is {:?}",
            self.id, best_branch
        );

        best_branch.resource_counts.get_resource(Resource::Geode)
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let blueprints = lines
            .iter()
            .map(|line| Blueprint::from_str(line).unwrap())
            .collect_vec();

        blueprints
            .iter()
            .map(|blueprint| blueprint.simulate(24))
            .max()
            .unwrap()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/19.txt");
    let input = include_str!("../../inputs/19.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
