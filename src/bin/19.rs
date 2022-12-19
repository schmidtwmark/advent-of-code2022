use aoc::Solver;
use hashbrown::HashMap;
use itertools::Itertools;
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BlueprintElement {
    costs: Vec<(Resource, usize)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    elements: HashMap<Resource, BlueprintElement>,
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
            let costs = costs_str
                .split(" and ")
                .map(|cost| {
                    let (count, resource) = scanf!(cost, "{} {}", usize, str).unwrap();
                    let resource = Resource::from_str(resource).unwrap();
                    (resource, count)
                })
                .collect_vec();
            (resource_type, BlueprintElement { costs })
        });

        Ok(Self {
            id,
            elements: elements.collect(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Branch {
    minute: usize,
    resource_counts: [usize; 4],
    robot_counts: [usize; 4],
    robot_in_production: Option<Resource>,
}

impl Branch {
    fn new() -> Self {
        Self {
            minute: 1,
            resource_counts: [0; 4],
            robot_counts: [0; 4],
            robot_in_production: None,
        }
    }
}

impl Blueprint {
    fn simulate(&self, minutes: usize) {
        let mut minute = 1;

        let mut branches = vec![Branch::new()];

        while minute < 24 {
            minute += 1;
        }
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let blueprints = lines
            .iter()
            .map(|line| Blueprint::from_str(line).unwrap())
            .collect_vec();

        Default::default()
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
