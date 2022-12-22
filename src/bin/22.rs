use std::{default, fmt::Display};

use aoc::Solver;
use itertools::Itertools;

use aoc::Grid;
use log::{debug, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

impl Direction {
    fn from(dir: (isize, isize)) -> Self {
        match dir {
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            _ => panic!("Unexpected direction: {:?}", dir),
        }
    }

    fn score(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Open,
    Player(Direction),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "⬛️"),
            Tile::Wall => write!(f, "🟥"),
            Tile::Open => write!(f, "⬜️"),
            Tile::Player(dir) => match dir {
                Direction::Up => write!(f, "️🟦"),
                Direction::Down => write!(f, "🟩"),
                Direction::Left => write!(f, "🟨"),
                Direction::Right => write!(f, "🟪"),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Command {
    Clockwise,
    Counterclockwise,
    Move(usize),
}

fn parse(lines: &[&str]) -> (Grid<Tile>, Vec<Command>) {
    let (grid_str, commands_str) = lines.split(|s| s.is_empty()).collect_tuple().unwrap();

    debug!("Grid: {:?}", grid_str);
    debug!("Commands: {:?}", commands_str);

    let width = grid_str.iter().map(|s| s.len()).max().unwrap();
    let height = grid_str.len();

    debug!("Grid with dimensions ({}, {})", width, height);

    let mut grid = Grid::new_empty(width, height);

    for (row, line) in grid_str.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => *grid.mut_at((col, row)) = Tile::Wall,
                '.' => *grid.mut_at((col, row)) = Tile::Open,
                ' ' => (), // Already set to empty
                _ => panic!("Unexpected character: {}", c),
            }
        }
    }

    assert_eq!(commands_str.len(), 1);
    let command_str = commands_str[0].chars().collect_vec();
    let mut commands = Vec::new();

    let mut idx = 0;
    let mut temp = String::new();

    let f = |temp: &mut String, commands: &mut Vec<Command>| {
        if !temp.is_empty() {
            commands.push(Command::Move(temp.parse().unwrap()));
            temp.clear();
        }
    };

    while idx < command_str.len() {
        match command_str[idx] {
            'R' => {
                f(&mut temp, &mut commands);
                commands.push(Command::Clockwise);
            }
            'L' => {
                f(&mut temp, &mut commands);
                commands.push(Command::Counterclockwise);
            }
            '0'..='9' => {
                temp.push(command_str[idx]);
            }
            _ => panic!("Unexpected character: {}", command_str[idx]),
        }
        idx += 1;
    }
    f(&mut temp, &mut commands);

    (grid, commands)
}

fn draw(
    grid: &mut Grid<Tile>,
    current_col: usize,
    current_row: usize,
    current_direction: (isize, isize),
) {
    let dir = Direction::from(current_direction);
    *grid.mut_at((current_col, current_row)) = Tile::Player(dir);
    debug!("{grid}");
    *grid.mut_at((current_col, current_row)) = Tile::Open;
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (mut grid, commands) = parse(lines);

        let mut current_col = grid.row(0).position(|t| *t == Tile::Open).unwrap();
        let mut current_row = 0;
        let mut current_dir = (1, 0);

        debug!("Starting at ({}, {})", current_col, current_row);

        for command in commands {
            match command {
                Command::Clockwise => {
                    let (x, y) = current_dir;
                    current_dir = (-y, x);
                }
                Command::Counterclockwise => {
                    let (x, y) = current_dir;
                    current_dir = (y, -x);
                }
                Command::Move(n) => {
                    for _ in 0..n {
                        let (x, y) = current_dir;
                        let mut next_col =
                            ((current_col as isize + x).rem_euclid(grid.width as isize)) as usize;
                        let mut next_row =
                            ((current_row as isize + y).rem_euclid(grid.height as isize)) as usize;

                        // Wrap around if out of bounds or tile is empty
                        let next_tile = grid.at((next_col, next_row));
                        (current_col, current_row) = match next_tile {
                            Tile::Open => (next_col, next_row),
                            Tile::Wall => (current_col, current_row),
                            Tile::Empty => {
                                let mut last_valid_position = (current_col, current_row);
                                loop {
                                    next_col = ((next_col as isize + x)
                                        .rem_euclid(grid.width as isize))
                                        as usize;
                                    next_row = ((next_row as isize + y)
                                        .rem_euclid(grid.height as isize))
                                        as usize;
                                    let next_tile = grid.at((next_col, next_row));
                                    match next_tile {
                                        Tile::Open => {
                                            last_valid_position = (next_col, next_row);
                                            break;
                                        }
                                        Tile::Wall => {
                                            break;
                                        }
                                        Tile::Empty => (),
                                        Tile::Player(_) => panic!("Unexpected player tile"),
                                    }
                                }

                                last_valid_position
                            }
                            Tile::Player(_) => panic!("Unexpected player tile"),
                        };
                    }
                }
            }
            debug!(
                "({}, {}) with direction {:?} after command {command:?}",
                current_col, current_row, current_dir
            );
        }

        draw(&mut grid, current_col, current_row, current_dir);
        let final_dir = Direction::from(current_dir);
        info!(
            "Final position: ({}, {}) with direction {}",
            current_col, current_row, final_dir
        );

        let final_col = current_col + 1;
        let final_row = current_row + 1;

        1000 * final_row + 4 * final_col + final_dir.score()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/22.txt");
    let sample_2 = include_str!("../../samples/22_1.txt");
    let input = include_str!("../../inputs/22.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 6032),
        aoc::Input::new_sample(sample_2, 1038),
        aoc::Input::new_final(input), // 39208 is too low
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
