#![feature(mixed_integer_ops)]

use aoc::Solver;
use im::{hashset, HashSet};
use itertools::Itertools;
use log::debug;
use std::collections::HashMap;
enum Push {
    Left,
    Right,
}

impl Push {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid push direction: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Tile {
    #[default]
    Air,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockShape {
    HLine,
    X,
    L,
    VLine,
    Square,
}

type Point = (usize, usize);

impl RockShape {
    fn shapes() -> [(RockShape, Vec<Point>); 5] {
        // Points start from bottom left
        [
            (RockShape::HLine, vec![(0, 0), (1, 0), (2, 0), (3, 0)]),
            (RockShape::X, vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
            (RockShape::L, vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)]),
            (RockShape::VLine, vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
            (RockShape::Square, vec![(0, 0), (1, 0), (0, 1), (1, 1)]),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RockState {
    Falling,
    Pushing,
    Settled,
}

struct Cave {
    fallen_rocks: HashSet<Point>,
    height: usize,
    pushers: Vec<Push>,
    pusher_idx: usize,
    shapes: [(RockShape, Vec<Point>); 5],
}

impl Cave {
    fn new(pushers: Vec<Push>) -> Self {
        Self {
            fallen_rocks: HashSet::new(),
            height: 0,
            pushers,
            pusher_idx: 0,
            shapes: RockShape::shapes(),
        }
    }

    fn simulate_rock(&mut self, rock_idx: usize) {
        let (_shape, points) = &self.shapes[rock_idx % 5];
        let mut points = points
            .iter()
            .map(|(x, y)| (x + 2, y + 3 + self.height))
            .collect_vec();
        debug!("Points start at {:?}", points);

        let mut rock_state = RockState::Pushing;

        loop {
            match rock_state {
                RockState::Falling => {
                    let new_points = points
                        .iter()
                        .filter_map(|(x, y)| {
                            let new_y = y.checked_add_signed(-1)?;
                            Some((*x, new_y))
                        })
                        .collect_vec();
                    if new_points.len() == points.len()
                        && !new_points
                            .iter()
                            .any(|point| self.fallen_rocks.contains(point))
                    {
                        points = new_points;
                        rock_state = RockState::Pushing;
                    } else {
                        rock_state = RockState::Settled;
                    }
                }
                RockState::Pushing => {
                    let pusher = &self.pushers[self.pusher_idx];
                    self.pusher_idx = (self.pusher_idx + 1) % self.pushers.len();

                    let offset = match pusher {
                        Push::Left => -1,
                        Push::Right => 1,
                    };

                    let new_points = points
                        .iter()
                        .filter_map(|(x, y)| {
                            let new_x = x.checked_add_signed(offset)?;
                            if new_x < 7 {
                                Some((new_x, *y))
                            } else {
                                None
                            }
                        })
                        .collect_vec();
                    if new_points.len() == points.len()
                        && !new_points
                            .iter()
                            .any(|point| self.fallen_rocks.contains(point))
                    {
                        points = new_points;
                    }

                    rock_state = RockState::Falling
                }
                RockState::Settled => {
                    let max_y = points.iter().map(|(_, y)| y).max().unwrap();
                    self.height = std::cmp::max(self.height, *max_y + 1);
                    debug!(
                        "Rock {} settled {}, final points {:?}",
                        rock_idx, self.height, points
                    );

                    self.fallen_rocks.extend(points);
                    break;
                }
            }

            debug!("Points are {:?}", points);
        }
    }

    fn draw(&self) {
        let mut map = HashMap::new();
        for (x, y) in self.fallen_rocks.iter() {
            map.insert((x, y), Tile::Rock);
        }

        for y in (0..=self.height).rev() {
            print!("|");
            for x in 0..7 {
                let tile = map.get(&(&x, &y)).copied().unwrap_or_default();
                match tile {
                    Tile::Air => print!("."),
                    Tile::Rock => print!("#"),
                }
            }
            print!("|");
            println!();
        }
        println!("+{}+", "-".repeat(7));
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let line = lines[0];
        let pushers = line.chars().map(Push::from_char).collect_vec();

        let mut cave = Cave::new(pushers);
        let rock_count = 2022;

        for rock_idx in 0..rock_count {
            cave.simulate_rock(rock_idx);
            // cave.draw();
        }

        cave.height
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let line = lines[0];
        let pushers = line.chars().map(Push::from_char).collect_vec();
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/17.txt");
    let input = include_str!("../../inputs/17.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 3068),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
