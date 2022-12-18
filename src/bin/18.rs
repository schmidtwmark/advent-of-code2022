use std::str::FromStr;

use aoc::Solver;
use hashbrown::HashSet;
use itertools::Itertools;
use log::debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { x, y, z })
    }
}
impl Cube {
    // Return the 6 neighbors of a cube
    fn get_neighbors(&self) -> impl Iterator<Item = Cube> + Clone + '_ {
        [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ]
        .iter()
        .map(move |(dx, dy, dz)| Cube {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        })
    }
}

fn count_surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes.iter().fold(0, |mut acc, cube| {
        let present_neighbors = cube.get_neighbors().filter(|c| cubes.contains(c));

        debug!(
            "{:?} -> {:?}",
            cube,
            present_neighbors.clone().collect_vec()
        );

        acc += 6 - present_neighbors.count();
        acc
    })
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let cubes: HashSet<Cube> = lines
            .iter()
            .map(|line| line.parse::<Cube>().unwrap())
            .collect();
        debug!("{:?}", cubes);

        count_surface_area(&cubes)
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/18.txt");
    let sample_2 = include_str!("../../samples/18_2.txt");
    let input = include_str!("../../inputs/18.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 64),
        aoc::Input::new_sample(sample_2, 10),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 58),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
