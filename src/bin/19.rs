use aoc::Solver;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use log::{debug, info};
use sscanf::scanf;
use std::{fmt::Display, str::FromStr};

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
                let (count, resource) = scanf!(cost, "{} {}", u8, str).unwrap();
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

impl Blueprint {
    fn get_values(&self) -> [u32; 4] {
        let ore_value = 1;
        let clay_value = self.elements[&Resource::Clay].get_resource(Resource::Ore) as u32;
        let obsidian_value = {
            let obsidian_costs = &self.elements[&Resource::Obsidian];
            obsidian_costs.get_resource(Resource::Ore) as u32
                + obsidian_costs.get_resource(Resource::Clay) as u32 * clay_value
        };

        let geode_value = {
            let geode_costs = &self.elements[&Resource::Geode];
            geode_costs.get_resource(Resource::Ore) as u32
                + geode_costs.get_resource(Resource::Clay) as u32 * clay_value
                + geode_costs.get_resource(Resource::Obsidian) as u32 * obsidian_value
        };

        [ore_value, clay_value, obsidian_value, geode_value]
    }
}
fn score_branch(resource_values: &[u32; 4], branch: &Branch, minutes_left: usize) -> u32 {
    let mut score = 0;
    // for (resource, count) in branch.resource_counts.resources.iter().enumerate() {
    //     score += *count as u32 * resource_values[resource];
    // }

    for (resource, count) in branch.robot_counts.resources.iter().enumerate() {
        score += *count as u32 * resource_values[resource] * minutes_left as u32;
    }
    score
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Branch {
    resource_counts: ResourceCounts,
    robot_counts: ResourceCounts,
    robot_in_production: Option<Resource>,
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ Resouces: [{}], robot_counts: [{}], robot_in_production: {:?} }}",
            self.resource_counts
                .resources
                .map(|r| r.to_string())
                .join(", "),
            self.robot_counts
                .resources
                .map(|r| r.to_string())
                .join(", "),
            self.robot_in_production
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ResourceCounts {
    resources: [u8; 4],
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

    fn from(iterator: impl Iterator<Item = (Resource, u8)>) -> Self {
        let mut resources = [0u8; 4];
        for (resource, count) in iterator {
            resources[resource as usize] = count;
        }
        Self { resources }
    }

    fn add_resource(&mut self, resource: Resource, count: u8) {
        self.resources[resource as usize] += count;
    }

    fn get_resource(&self, resource: Resource) -> u8 {
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

    fn simulate(&mut self, blueprint: &Blueprint, _minute: usize) -> impl Iterator<Item = Self> {
        let mut new_branches = vec![];

        self.resource_counts.add(&self.robot_counts);

        if let Some(robot) = self.robot_in_production {
            self.robot_counts.add_resource(robot, 1);
            self.robot_in_production = None;
        }

        for (robot, costs) in blueprint.elements.iter() {
            if costs.less_than_equal(&self.resource_counts) {
                let mut new_branch = *self;
                new_branch.robot_in_production = Some(*robot);
                new_branch.resource_counts.subtract(costs);
                new_branches.push(new_branch);
            }
        }
        // Do nothing except gain resources
        new_branches.push(*self);
        new_branches.into_iter()
    }
}

impl Blueprint {
    fn simulate(&self, minutes: usize) -> u8 {
        let mut minute = 1;

        let resource_values = self.get_values();
        debug!("Blueprint {} has values {:?}", self.id, resource_values);

        let mut branches = HashSet::new();
        branches.insert(Branch::new());

        while minute <= minutes {
            let minutes_left = minutes - minute;
            let mut new_branches = HashSet::new();
            for mut branch in branches {
                new_branches.extend(
                    branch
                        .simulate(self, minute)
                        .map(|b| (b, score_branch(&resource_values, &b, minutes_left))),
                );
            }

            while new_branches.len() > 100000 {
                let before_filter = new_branches.len();
                debug!(
                    "Minute {}: Branch overflow! {} branches",
                    minute, before_filter
                );

                let (best_branch, best_score) =
                    new_branches.iter().max_by_key(|(_, s)| *s).unwrap();
                let (_worst_branch, worst_score) =
                    new_branches.iter().min_by_key(|(_, s)| *s).unwrap();

                let best_geodes = best_branch.resource_counts.get_resource(Resource::Geode);
                debug!(
                    "Minute {}: Best branch {} has {} geodes and score {}",
                    minute, best_branch, best_geodes, best_score
                );

                let mean = (best_score + worst_score) / 2;

                new_branches = new_branches.drain_filter(|(_b, s)| *s >= mean).collect();
                if new_branches.len() == before_filter {
                    break;
                }
            }

            branches = new_branches.iter().map(|(b, _)| *b).collect();
            debug!("Minute {}: {} branches", minute, branches.len());

            minute += 1;
        }

        let best_branch = branches
            .iter()
            .max_by_key(|b| b.resource_counts.get_resource(Resource::Geode))
            .unwrap();

        debug!(
            "For blueprint #{}, the best branch has {} geodes. It is {}",
            self.id,
            best_branch.resource_counts.get_resource(Resource::Geode),
            best_branch
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

        const MINUTES: usize = 24;

        blueprints
            .iter()
            .map(|blueprint| blueprint.simulate(MINUTES) as usize * blueprint.id)
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let blueprints = lines
            .iter()
            .map(|line| Blueprint::from_str(line).unwrap())
            .collect_vec();

        let blueprints = blueprints.into_iter().take(3).collect_vec();

        const MINUTES: usize = 32;

        blueprints
            .iter()
            .map(|blueprint| blueprint.simulate(MINUTES) as usize)
            .product()
    }
}

fn main() {
    let sample = include_str!("../../samples/19.txt");
    let input = include_str!("../../inputs/19.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 33),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 3472),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
