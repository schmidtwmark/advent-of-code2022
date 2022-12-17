#![feature(mixed_integer_ops)]

use aoc::Solver;
use itertools::Itertools;
use log::{debug, info};
use std::collections::HashMap;
use std::collections::HashSet;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    column_offsets: [Option<usize>; 7],
    extra_height: usize,
    pushers: Vec<Push>,
    pusher_idx: usize,
    shapes: [(RockShape, Vec<Point>); 5],
    cache: hashbrown::HashMap<(RockShape, usize, [Option<usize>; 7]), (usize, usize)>,
}

impl Cave {
    fn new(pushers: Vec<Push>) -> Self {
        Self {
            fallen_rocks: HashSet::new(),
            height: 0,
            column_offsets: [None; 7],
            extra_height: 0,
            pushers,
            pusher_idx: 0,
            shapes: RockShape::shapes(),
            cache: hashbrown::HashMap::new(),
        }
    }

    fn simulate_rock(&mut self, rock_idx: usize, target_rocks: usize) -> usize {
        let (shape, points) = &self.shapes[rock_idx % 5];
        let mut points = points
            .iter()
            .map(|(x, y)| (x + 2, y + 3 + self.height))
            .collect_vec();

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
                    let old_height = self.height;
                    self.height = std::cmp::max(self.height, *max_y + 1);
                    let difference = self.height - old_height;
                    for col in 0..7 {
                        let new_y = points
                            .iter()
                            .filter(|(x, _)| x == &col)
                            .map(|(_, y)| self.height - y - 1)
                            .min();

                        let old_offset = self.column_offsets[col];
                        self.column_offsets[col] = match (new_y, old_offset) {
                            (Some(new_y), Some(old_offset)) => {
                                Some(std::cmp::min(old_offset + difference, new_y))
                            }
                            (Some(new_y), None) => Some(new_y),
                            (None, Some(old_offset)) => Some(old_offset + difference),
                            (None, None) => None,
                        }
                        // self.column_offsets[col] = std::cmp::min(old_offset, new_y);
                    }
                    debug!(
                        "Rock {} settled {}, final points {:?}",
                        rock_idx, self.height, points
                    );
                    self.fallen_rocks.extend(points);

                    // return rock_idx + 1;

                    let cache_key = (*shape, self.pusher_idx, self.column_offsets);
                    let new_rock_idx = rock_idx
                        + if let Some((cache_idx, cache_height)) = self.cache.get(&cache_key) {
                            let repeats =
                                (target_rocks - cache_idx - 1) / (rock_idx - cache_idx) - 1;
                            self.extra_height += (self.height - cache_height) * repeats;
                            info!(
                                "Rock {} Cache hit for {:?} => ({}, {}), repeating {} times",
                                rock_idx, cache_key, cache_idx, cache_height, repeats
                            );
                            (rock_idx - cache_idx) * repeats + 1
                        } else {
                            info!(
                                "Rock {} Cache miss for {:?}, inserting ({}, {})",
                                rock_idx, cache_key, rock_idx, self.height
                            );
                            self.cache.insert(cache_key, (rock_idx, self.height));
                            1
                        };
                    debug!(
                        "Rock {} height is {}, total height is {}, next: {}",
                        rock_idx,
                        self.height,
                        self.get_total_height(),
                        new_rock_idx
                    );
                    return new_rock_idx;
                }
            }
        }
    }

    fn draw(&self) {
        let mut map = HashMap::new();
        for (x, y) in self.fallen_rocks.iter() {
            map.insert((x, y), Tile::Rock);
        }

        for y in (0..=self.height).rev() {
            let mut line = String::with_capacity(9);
            line.push('|');
            for x in 0..7 {
                let tile = map.get(&(&x, &y)).copied().unwrap_or_default();
                match tile {
                    Tile::Air => line.push('.'),
                    Tile::Rock => line.push('#'),
                }
            }
            line.push('|');
            debug!("{}", line);
        }
        debug!("+{}+", "-".repeat(7));
    }

    fn cull_rocks(&mut self) {
        let offset = 100;
        if self.fallen_rocks.len() > 1000 && self.height > 2 * offset {
            let barrier = self.height - offset;
            self.fallen_rocks.retain(|(_, y)| *y > barrier);
        }
    }

    fn get_total_height(&self) -> usize {
        self.height + self.extra_height
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let line = lines[0];
        let pushers = line.chars().map(Push::from_char).collect_vec();

        let mut cave = Cave::new(pushers);
        let rock_count = 2022;
        let mut rock_idx = 0;
        while rock_idx < rock_count {
            rock_idx = cave.simulate_rock(rock_idx, rock_count);
            cave.cull_rocks();
            // cave.draw()
        }
        info!("Done, rock_idx = {}", rock_idx);

        cave.get_total_height()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let line = lines[0];
        let pushers = line.chars().map(Push::from_char).collect_vec();

        let final_rock_count: usize = 1000000000000; // one trillion

        let mut cave = Cave::new(pushers);
        let mut rock_idx = 0;

        while rock_idx < final_rock_count {
            rock_idx = cave.simulate_rock(rock_idx, final_rock_count);
            cave.cull_rocks();
        }
        cave.get_total_height()
        // 1565242165215 is too high
        // 1565242165189 is too low
        // 1565242165191 is too low
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
        aoc::Input::new_sample(sample, 1514285714288),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
