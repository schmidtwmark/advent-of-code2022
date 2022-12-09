#![feature(exclusive_range_pattern)]
use std::str::FromStr;

use aoc::Solver;
use itertools::Itertools;
use log::debug;
use std::cmp::Ordering;
use std::collections::HashSet;

use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn move_by(&mut self, command: &mut Command) -> bool {
        match command.direction {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        command.distance -= 1;
        command.distance > 0
    }

    fn abs_delta(&self, other: &Position) -> (usize, usize) {
        (
            (self.x - other.x).unsigned_abs(),
            (self.y - other.y).unsigned_abs(),
        )
    }

    fn shift(tail: &isize, head: &isize) -> isize {
        match tail.cmp(head) {
            Ordering::Greater => tail - 1,
            Ordering::Less => tail + 1,
            Ordering::Equal => *tail,
        }
    }

    fn step_tail_closer(&mut self, head: &Position) {
        let (dx, dy) = self.abs_delta(head);
        match (dx, dy) {
            (0, 0) => (),
            (0, 2..usize::MAX) => self.y = Self::shift(&self.y, &head.y),
            (2..usize::MAX, 0) => self.x = Self::shift(&self.x, &head.x),
            (1..usize::MAX, 2..usize::MAX) => {
                self.x = Self::shift(&self.x, &head.x);
                self.y = Self::shift(&self.y, &head.y);
            }
            (2..usize::MAX, 1..usize::MAX) => {
                self.x = Self::shift(&self.x, &head.x);
                self.y = Self::shift(&self.y, &head.y);
            }
            _ => (),
        }
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_whitespace().collect_tuple().unwrap();
        let distance = distance.parse().unwrap();
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction: {}", direction),
        };
        Ok(Command {
            direction,
            distance,
        })
    }
}

fn debug_rope(rope: &[Position]) {
    for (idx, position) in rope.iter().enumerate() {
        if idx == 0 {
            debug!("Head: {}", position);
        } else {
            debug!("{}: {}", idx, position);
        }
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let mut commands = lines
            .iter()
            .map(|s| Command::from_str(s).unwrap())
            .collect_vec();
        debug!("{:?}", commands);

        let mut head = Position::new(0, 0);
        let mut tail = head;

        let mut visited = HashSet::new();
        visited.insert(tail);

        for command in &mut commands {
            loop {
                head.move_by(command);
                tail.step_tail_closer(&head);
                visited.insert(tail);
                if command.distance == 0 {
                    break;
                }
            }
        }

        visited.len()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let mut commands = lines
            .iter()
            .map(|s| Command::from_str(s).unwrap())
            .collect_vec();
        debug!("{:?}", commands);

        let mut rope = [Position::new(0, 0); 10];

        let mut visited = HashSet::new();
        visited.insert(*rope.last().unwrap());

        for command in &mut commands {
            debug!("Command: {:?}", command);
            loop {
                rope[0].move_by(command);
                for (head_idx, tail_idx) in (0..rope.len()).tuple_windows() {
                    let (head_side, tail_side) = rope.split_at_mut(tail_idx);
                    let head = &mut head_side[head_idx];
                    let tail = &mut tail_side[0];
                    tail.step_tail_closer(head);
                }
                visited.insert(*rope.last().unwrap());
                debug_rope(&rope);
                debug!("");
                if command.distance == 0 {
                    break;
                }
            }
            debug!("");
            debug!("");
        }

        visited.len()
    }
}

fn main() {
    let sample = include_str!("../../samples/9.txt");
    let sample_2 = include_str!("../../samples/9_2.txt");
    let input = include_str!("../../inputs/9.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 13),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 1),
        aoc::Input::new_sample(sample_2, 36),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
