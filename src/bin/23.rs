use std::fmt::Display;

use aoc::Solver;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use log::{debug, info};
use std::cmp::{max, min};

struct Board {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    elves: HashSet<(isize, isize)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if self.elves.contains(&(x, y)) {
                    write!(f, "🟦")?;
                } else {
                    write!(f, "⬛️")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    fn from_lines(lines: &[&str]) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let elves = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some((x as isize, y as isize)),
                    _ => None,
                })
            })
            .collect();
        Self {
            min_x: 0,
            max_x: width as isize,
            min_y: 0,
            max_y: height as isize,
            elves,
        }
    }

    fn count_empty_spaces(&self) -> usize {
        let mut count = 0;
        for x in self.min_x..=self.max_x {
            for y in self.min_y..=self.max_y {
                if !self.elves.contains(&(x, y)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn adjacent_elves(&self, x: isize, y: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    return None;
                }
                let new_x = x + dx;
                let new_y = y + dy;
                if self.elves.contains(&(new_x, new_y)) {
                    Some((new_x, new_y))
                } else {
                    None
                }
            })
    }

    fn elves_in_direction(&self, x: isize, y: isize, direction: Direction) -> Vec<(isize, isize)> {
        let deltas = match direction {
            Direction::North => [(0, -1), (-1, -1), (1, -1)],
            Direction::South => [(0, 1), (-1, 1), (1, 1)],
            Direction::East => [(1, 0), (1, -1), (1, 1)],
            Direction::West => [(-1, 0), (-1, -1), (-1, 1)],
        };

        deltas
            .iter()
            .filter_map(move |(dx, dy)| {
                let pos = (x + dx, y + dy);
                self.elves.get(&pos).copied()
            })
            .collect()
    }

    fn simulate(&mut self, round: usize) -> usize {
        let mut new_elves = HashSet::new();

        let directions_for_round = DIRECTIONS
            .iter()
            .cycle()
            .skip(round - 1)
            .take(4)
            .collect_vec();
        debug!("Directions for round {}: {:?}", round, directions_for_round);

        let proposed_moves = self.elves.iter().map(|(x, y)| {
            let adjacent_elves = self.adjacent_elves(*x, *y).collect_vec();
            if adjacent_elves.is_empty() {
                debug!("Elf at ({}, {}) has no adjacent elves", x, y);
                return ((*x, *y), (*x, *y));
            }
            let proposed_direction = directions_for_round.iter().find(|direction| {
                let elves_in_direction = self.elves_in_direction(*x, *y, ***direction);
                elves_in_direction.is_empty()
            });
            if let Some(proposed_direction) = proposed_direction {
                debug!(
                    "Elf at ({}, {}) proposed direction: {:?}",
                    x, y, proposed_direction
                );
                let (offset_x, offset_y) = proposed_direction.offset();
                ((*x + offset_x, *y + offset_y), (*x, *y))
            } else {
                ((*x, *y), (*x, *y))
            }
        });

        let mut proposed_map = HashMap::new();

        for (proposed_move, elf) in proposed_moves {
            proposed_map
                .entry(proposed_move)
                .or_insert_with(Vec::new)
                .push(elf);
        }

        debug!("Proposed moves: {:?}", proposed_map);
        let mut moved_elf_count = 0;

        for (proposed_move, elves) in proposed_map {
            if elves.len() == 1 {
                new_elves.insert(proposed_move);
                if proposed_move != elves[0] {
                    moved_elf_count += 1;
                }
            } else {
                for elf in elves {
                    new_elves.insert(elf);
                }
            }
        }
        self.min_x = isize::MAX;
        self.max_x = isize::MIN;
        self.min_y = isize::MAX;
        self.max_y = isize::MIN;

        for elf in new_elves.iter() {
            let (x, y) = *elf;
            self.min_x = min(self.min_x, x);
            self.max_x = max(self.max_x, x);
            self.min_y = min(self.min_y, y);
            self.max_y = max(self.max_y, y);
        }

        self.elves = new_elves;
        moved_elf_count
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let mut board = Board::from_lines(lines);
        info!("{}", board);

        for round in 1..=10 {
            let moved = board.simulate(round);
            info!("Round {}: {} elves moved", round, moved);
            debug!("{}", board);
        }
        board.count_empty_spaces()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let mut board = Board::from_lines(lines);
        info!("{}", board);

        let mut round = 1;
        loop {
            let moved = board.simulate(round);
            info!("Round {}: {} elves moved", round, moved);
            debug!("{}", board);

            if moved == 0 {
                break;
            }
            round += 1;
        }
        round
    }
}

fn main() {
    let sample = include_str!("../../samples/23.txt");
    let sample_2 = include_str!("../../samples/23_1.txt");
    let input = include_str!("../../inputs/23.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 110),
        aoc::Input::new_sample(sample_2, 25),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 20),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
