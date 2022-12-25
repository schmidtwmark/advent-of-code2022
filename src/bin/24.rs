use aoc::Solver;
use hashbrown::{HashMap, HashSet};
use log::debug;

type Position = (isize, isize);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct State {
    blizzards: HashMap<Position, Vec<Direction>>,
    player: Position,
    goal: Position,
    x_range: (isize, isize),
    y_range: (isize, isize),
}

impl State {
    fn from_lines(lines: &[&str]) -> Self {
        let x_range = (1_isize, lines[0].len() as isize - 2);
        let y_range = (1_isize, lines.len() as isize - 2);

        let start = lines[0];
        let end = lines[lines.len() - 1];

        let player = (start.find('.').unwrap() as isize, 0);
        let goal = (end.find('.').unwrap() as isize, lines.len() as isize - 1);
        let mut blizzards = HashMap::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if let Some(dir) = match char {
                    '>' => Some(Direction::Right),
                    '<' => Some(Direction::Left),
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    _ => None,
                } {
                    blizzards
                        .entry((x as isize, y as isize))
                        .or_insert_with(Vec::new)
                        .push(dir);
                }
            }
        }

        State {
            blizzards,
            player,
            goal,
            x_range,
            y_range,
        }
    }

    fn wrap_pos(&self, pos: Position) -> Position {
        let (mut new_x, mut new_y) = pos;
        let (min_x, max_x) = self.x_range;
        let (min_y, max_y) = self.y_range;

        if new_x == min_x - 1 {
            new_x = max_x;
        } else if new_x == max_x + 1 {
            new_x = min_x;
        }

        if new_y == min_y - 1 {
            new_y = max_y;
        } else if new_y == max_y + 1 {
            new_y = min_y;
        }

        (new_x, new_y)
    }

    fn in_bounds(&self, pos: Position) -> bool {
        let (x, y) = pos;
        let (min_x, max_x) = self.x_range;
        let (min_y, max_y) = self.y_range;

        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }

    fn new_blizzards(&self) -> HashMap<Position, Vec<Direction>> {
        let mut blizzards = HashMap::new();

        for (pos, dir) in &self.blizzards {
            for dir in dir {
                let pos = match dir {
                    Direction::Up => (pos.0, pos.1 - 1),
                    Direction::Down => (pos.0, pos.1 + 1),
                    Direction::Left => (pos.0 - 1, pos.1),
                    Direction::Right => (pos.0 + 1, pos.1),
                };

                blizzards
                    .entry(self.wrap_pos(pos))
                    .or_insert_with(Vec::new)
                    .push(*dir);
            }
        }

        blizzards
    }

    fn time_to_goal(&mut self) -> usize {
        let mut minute = 1;
        let mut branches = HashSet::new();
        branches.insert(self.player);

        loop {
            // debug!("Minute: {}, {} branches", minute, branches.len());
            let mut new_branches = HashSet::new();
            let new_blizzards = self.new_blizzards();

            for branch in branches.iter() {
                if !new_blizzards.contains_key(branch) {
                    let new_branch = *branch;
                    new_branches.insert(new_branch); // Stay put
                }

                for direction in DIRECTIONS {
                    let new_branch = match direction {
                        Direction::Up => (branch.0, branch.1 - 1),
                        Direction::Down => (branch.0, branch.1 + 1),
                        Direction::Left => (branch.0 - 1, branch.1),
                        Direction::Right => (branch.0 + 1, branch.1),
                    };

                    if new_branch == self.goal {
                        self.blizzards = new_blizzards;
                        return minute;
                    }

                    if self.in_bounds(new_branch) {
                        if new_blizzards.contains_key(&new_branch) {
                            // If the player moves into a blizzard, stop this branch
                            continue;
                        }

                        new_branches.insert(new_branch);
                    }
                }
            }

            self.blizzards = new_blizzards;
            branches = new_branches;
            minute += 1;
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let mut state = State::from_lines(lines);

        debug!("Initial state: {:?}", state);

        state.time_to_goal()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let mut state = State::from_lines(lines);
        let start_point = state.player;
        let end_point = state.goal;

        debug!("Initial state: {:?}", state);
        let first_run = state.time_to_goal();
        debug!("Took {} minutes to reach the goal", first_run);

        state.player = end_point;
        state.goal = start_point;
        let second_run = state.time_to_goal();
        debug!("Took {} minutes to reach the start", second_run);

        state.player = start_point;
        state.goal = end_point;
        let third_run = state.time_to_goal();
        debug!("Took {} minutes to reach the goal", third_run);

        first_run + second_run + third_run
    }
}

fn main() {
    let sample = include_str!("../../samples/24.txt");
    let sample_2 = include_str!("../../samples/24_1.txt");
    let input = include_str!("../../inputs/24.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 18),
        aoc::Input::new_sample(sample_2, 10),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 54),
        aoc::Input::new_final(input), // 844 is too low // 846 is too low
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
