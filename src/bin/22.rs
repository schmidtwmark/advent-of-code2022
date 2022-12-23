#![feature(mixed_integer_ops)]
#![feature(let_chains)]
use std::{collections::VecDeque, default, fmt::Display};

use aoc::Solver;
use hashbrown::HashMap;
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

    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn rotate(&self, clockwise: bool) -> Self {
        match self {
            Direction::Up => {
                if clockwise {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Down => {
                if clockwise {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Left => {
                if clockwise {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
            Direction::Right => {
                if clockwise {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
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
    current_direction: Direction,
) {
    *grid.mut_at((current_col, current_row)) = Tile::Player(current_direction);
    debug!("{grid}");
    *grid.mut_at((current_col, current_row)) = Tile::Open;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Face {
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
}

impl Face {
    fn neighbor(&self, dir: Direction) -> Face {
        match self {
            Face::Top => match dir {
                Direction::Up => Face::Back,
                Direction::Down => Face::Front,
                Direction::Left => Face::Left,
                Direction::Right => Face::Right,
            },
            Face::Front => match dir {
                Direction::Up => Face::Top,
                Direction::Down => Face::Bottom,
                Direction::Left => Face::Left,
                Direction::Right => Face::Right,
            },
            Face::Left => match dir {
                Direction::Up => Face::Top,
                Direction::Down => Face::Bottom,
                Direction::Left => Face::Back,
                Direction::Right => Face::Front,
            },
            Face::Right => match dir {
                Direction::Up => Face::Top,
                Direction::Down => Face::Bottom,
                Direction::Left => Face::Front,
                Direction::Right => Face::Back,
            },
            Face::Bottom => match dir {
                Direction::Up => Face::Front,
                Direction::Down => Face::Back,
                Direction::Left => Face::Left,
                Direction::Right => Face::Right,
            },
            Face::Back => match dir {
                Direction::Up => Face::Top,
                Direction::Down => Face::Bottom,
                Direction::Left => Face::Right,
                Direction::Right => Face::Left,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Cube {
    faces: hashbrown::HashMap<Face, Grid<Tile>>,
    face_to_pos: hashbrown::HashMap<Face, (usize, usize)>, // (col, row) in the flat map
    flat_map: Grid<Tile>,
}

impl Cube {
    fn new(flat_map: Grid<Tile>) -> Cube {
        let face_length = num::integer::gcd(flat_map.width, flat_map.height);

        let mut faces = hashbrown::HashMap::new();
        let mut face_to_pos = hashbrown::HashMap::new();

        let horizontal_faces = flat_map.width / face_length;
        let vertical_faces = flat_map.height / face_length;

        let checker = |face: Face,
                       col: usize,
                       row: usize,
                       faces: &mut HashMap<Face, Grid<Tile>>,
                       face_to_pos: &mut HashMap<Face, (usize, usize)>| {
            let (actual_row, actual_col) = (col * face_length, row * face_length);
            if let Some(tile) = flat_map.get((actual_col, actual_row)) {
                if *tile != Tile::Empty {
                    face_to_pos.insert(face, (col, row));
                    faces.insert(
                        face,
                        flat_map.get_subgrid((actual_col, actual_row), face_length, face_length),
                    );
                }
            }
        };

        // First, find top
        let top_col = (0..horizontal_faces)
            .find(|col| {
                if let Some(tile) = flat_map.get((col * face_length, 0)) {
                    tile != &Tile::Empty
                } else {
                    false
                }
            })
            .expect("No top face found, cringe");

        face_to_pos.insert(Face::Top, (top_col, 0));
        faces.insert(
            Face::Top,
            flat_map.get_subgrid((top_col * face_length, 0), face_length, face_length),
        );
        info!("Found top face, {:?}", face_to_pos);

        let mut to_visit = VecDeque::new();
        to_visit.push_back(Face::Top);

        // Look at the neighbors of the top face
        while faces.len() < 6 {
            let face = to_visit.pop_front().unwrap();

            let (face_col, face_row) = face_to_pos[&face];

            let dirs = [Direction::Down, Direction::Left, Direction::Right];

            for dir in dirs {
                let target_face = face.neighbor(dir);
                let (dx, dy) = dir.offset();
                if dx == -1 && face_col == 0 {
                    continue;
                }
                let (new_face_col, new_face_row) = (
                    (face_col as isize + dx) as usize,
                    (face_row as isize + dy) as usize,
                );

                let (actual_col, actual_row) =
                    (new_face_col * face_length, new_face_row * face_length);
                if let Some(tile) = flat_map.get((actual_col, actual_row)) {
                    debug!(
                        "Checking face {:?} at ({}, {}), got tile {:?}",
                        target_face, actual_col, actual_row, tile
                    );
                    if *tile != Tile::Empty && !faces.contains_key(&target_face) {
                        info!(
                            "Setting face {:?} at ({}, {})",
                            target_face, new_face_col, new_face_row
                        );
                        face_to_pos.insert(target_face, (new_face_col, new_face_row));
                        faces.insert(
                            target_face,
                            flat_map.get_subgrid(
                                (actual_col, actual_row),
                                face_length,
                                face_length,
                            ),
                        );
                        to_visit.push_back(target_face);
                    } else {
                        debug!(
                            "Skipping face {:?} at ({}, {})",
                            target_face, new_face_col, new_face_row
                        );
                    }
                }
            }
        }

        assert_eq!(faces.len(), 6);
        assert_eq!(face_to_pos.len(), 6);
        Cube {
            faces,
            face_to_pos,
            flat_map,
        }
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (mut grid, commands) = parse(lines);

        let mut current_col = grid.row(0).position(|t| *t == Tile::Open).unwrap();
        let mut current_row = 0;
        let mut current_dir = Direction::Right;

        debug!("Starting at ({}, {})", current_col, current_row);

        for command in commands {
            match command {
                Command::Clockwise => current_dir = current_dir.rotate(true),
                Command::Counterclockwise => current_dir = current_dir.rotate(false),
                Command::Move(n) => {
                    for _ in 0..n {
                        let (dx, dy) = current_dir.offset();
                        let mut next_col =
                            ((current_col as isize + dx).rem_euclid(grid.width as isize)) as usize;
                        let mut next_row =
                            ((current_row as isize + dy).rem_euclid(grid.height as isize)) as usize;

                        // Wrap around if out of bounds or tile is empty
                        let next_tile = grid.at((next_col, next_row));
                        (current_col, current_row) = match next_tile {
                            Tile::Open => (next_col, next_row),
                            Tile::Wall => (current_col, current_row),
                            Tile::Empty => {
                                let mut last_valid_position = (current_col, current_row);
                                loop {
                                    next_col = ((next_col as isize + dx)
                                        .rem_euclid(grid.width as isize))
                                        as usize;
                                    next_row = ((next_row as isize + dy)
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
        info!(
            "Final position: ({}, {}) with direction {}",
            current_col, current_row, current_dir
        );

        let final_col = current_col + 1;
        let final_row = current_row + 1;

        1000 * final_row + 4 * final_col + current_dir.score()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (mut grid, commands) = parse(lines);

        let mut current_col = grid.row(0).position(|t| *t == Tile::Open).unwrap();
        let mut current_row = 0;
        let mut current_dir = Direction::Right;

        let cube = Cube::new(grid);

        debug!("Starting at ({}, {})", current_col, current_row);

        for command in commands {
            match command {
                Command::Clockwise => {
                    current_dir = current_dir.rotate(true);
                }
                Command::Counterclockwise => {
                    current_dir = current_dir.rotate(false);
                }
                Command::Move(n) => {
                    for _ in 0..n {
                        let (dx, dy) = current_dir.offset();
                        let next_col = current_col as isize + dx;
                        let next_row = current_row as isize + dy;

                        // Wrap around if out of bounds or tile is empty
                        // (current_col, current_row, current_dir) = if next_col >= 0
                        //     && next_col < grid.width as isize
                        //     && next_row >= 0
                        //     && next_row < grid.height as isize
                        // {
                        //     todo!()
                        // }
                    }
                }
            }
            debug!(
                "({}, {}) with direction {:?} after command {command:?}",
                current_col, current_row, current_dir
            );
        }

        info!(
            "Final position: ({}, {}) with direction {}",
            current_col, current_row, current_dir
        );

        let final_col = current_col + 1;
        let final_row = current_row + 1;

        1000 * final_row + 4 * final_col + current_dir.score()
    }
}

fn main() {
    let sample = include_str!("../../samples/22.txt");
    let sample_2 = include_str!("../../samples/22_1.txt");
    let input = include_str!("../../inputs/22.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 6032),
        aoc::Input::new_sample(sample_2, 1038),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 5031),
        aoc::Input::new_sample(sample_2, 12056),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
