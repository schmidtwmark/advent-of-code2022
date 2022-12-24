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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Wall,
    Open(Option<Direction>),
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Empty
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => "⬛️",
                Tile::Wall => "🟥",
                Tile::Open(dir) => {
                    if let Some(dir) = dir {
                        match dir {
                            Direction::Up => "️🟦",
                            Direction::Down => "🟩",
                            Direction::Left => "🟨",
                            Direction::Right => "🟪",
                        }
                    } else {
                        "⬜️"
                    }
                }
            }
        )
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

    let width = grid_str.iter().map(|s| s.len()).max().unwrap();
    let height = grid_str.len();

    debug!("Grid with dimensions ({}, {})", width, height);

    let mut grid = Grid::new_empty(width, height);

    for (row, line) in grid_str.iter().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => *grid.mut_at((col, row)) = Tile::Wall,
                '.' => *grid.mut_at((col, row)) = Tile::Open(None),
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

fn wrap_around_cube(
    is_sample: bool,
    cube_width: usize,
    (x, y): (isize, isize),
    direction: Direction,
) -> (usize, usize, Direction) {
    if is_sample {
        wrap_around_cube_sample(4, (x, y), direction)
    } else {
        let input = (x, y, direction);
        debug!("wrap_around_cube({:?})", input);
        let output = wrap_around_cube_final(cube_width, (x, y), direction);
        debug!("wrap_around_cube({:?}) = {:?}", input, output);
        output
    }
}

fn wrap_around_cube_final(
    cube_width: usize,
    (x, y): (isize, isize),
    direction: Direction,
) -> (usize, usize, Direction) {
    let cube_width = cube_width as isize;
    let face_x = x % cube_width;
    let face_y = y % cube_width;

    debug!(
        "face_x = {}, face_y = {}, cube_width = {}",
        face_x, face_y, cube_width
    );
    if y == -1 {
        assert_eq!(direction, Direction::Up);
        // off top edge of top
        if x >= cube_width && x < cube_width * 2 {
            // End up on back, going right
            return (0, (cube_width * 3 + face_x) as usize, Direction::Right);
        } else if x >= cube_width * 2 && x < cube_width * 3 {
            // Off top edge of right, end up on back, bottom edge going up
            return (
                (face_x) as usize,
                (cube_width * 4 - 1) as usize,
                Direction::Up,
            );
        } else {
            panic!("Unexpected x value: {}", x);
        }
    }

    if y == cube_width * 4 && x >= 0 && x < cube_width {
        assert_eq!(direction, Direction::Down);
        // Bottom of back, end up on top edge of right
        return ((cube_width * 2 + face_x) as usize, 0, Direction::Down);
    }

    if x == -1 {
        assert_eq!(direction, Direction::Left);
        // Left edge of left, end up on left edge of top
        if y >= cube_width * 2 && y < cube_width * 3 {
            return (
                cube_width as usize,
                (cube_width - face_y - 1) as usize,
                Direction::Right,
            );
        } else if y >= cube_width * 3 && y < cube_width * 4 {
            // Left edge of back, end up on top edge of top
            return ((cube_width + face_y) as usize, 0, Direction::Down);
        } else {
            panic!("Unexpected y value: {}", y);
        }
    }

    if x == cube_width * 3 && y >= 0 && y < cube_width {
        assert_eq!(direction, Direction::Right);
        // Right edge of right, end up on right edge of bottom
        return (
            (cube_width * 2 - 1) as usize,
            (cube_width * 3 - face_y - 1) as usize,
            Direction::Left,
        );
    }

    if x >= cube_width && x < cube_width * 2 && y >= cube_width * 3 && y < cube_width * 4 {
        return match direction {
            // Right side of back to bottom side of bottom
            Direction::Right => (
                (cube_width + face_y) as usize,
                (cube_width * 3 - 1) as usize,
                Direction::Up,
            ),
            // Bottom side of bottom to bottom side of back
            Direction::Down => (
                (cube_width - 1) as usize,
                (cube_width * 3 + face_x) as usize,
                Direction::Left,
            ),
            _ => panic!("Unexpected wrap around {}: ({}, {})", direction, x, y),
        };
    }

    if y >= cube_width && y < cube_width * 2 && x >= cube_width * 2 && x < cube_width * 3 {
        return match direction {
            Direction::Down => (
                // Bottom side of right, to right side of front
                (cube_width * 2 - 1) as usize,
                (cube_width + face_x) as usize,
                Direction::Left,
            ),
            Direction::Right => (
                // Right side of front, to bottom side of right
                (cube_width * 2 + face_y) as usize,
                (cube_width - 1) as usize,
                Direction::Up,
            ),
            _ => panic!("Unexpected wrap around {}: ({}, {})", direction, x, y),
        };
    }

    if x >= 0 && x < cube_width && y >= cube_width && y < cube_width * 2 {
        return match direction {
            Direction::Up => (
                // left cube up to front cube left side
                (cube_width) as usize,
                (cube_width + face_x) as usize,
                Direction::Right,
            ),
            Direction::Left => (
                // Left side of front to top side of left
                (face_y) as usize,
                (cube_width * 2) as usize,
                Direction::Down,
            ),
            _ => panic!("Unexpected wrap around {}: ({}, {})", direction, x, y),
        };
    }

    if x == cube_width - 1 && y >= 0 && y < cube_width {
        // Left edge of top to right edge of left
        assert_eq!(direction, Direction::Left);
        return (0, (cube_width * 3 - face_y - 1) as usize, Direction::Right);
    }

    if x == cube_width * 2 && y >= cube_width * 2 && y < cube_width * 3 {
        // Right edge of bottom to right edge of right
        assert_eq!(direction, Direction::Right);
        return (
            (cube_width * 3 - 1) as usize,
            (cube_width - face_y - 1) as usize,
            Direction::Left,
        );
    }

    panic!("Unexpected wrap around {}: ({}, {})", direction, x, y);
}

fn wrap_around_cube_sample(
    cube_width: usize,
    (x, y): (isize, isize),
    direction: Direction,
) -> (usize, usize, Direction) {
    let cube_width = cube_width as isize;
    let face_x = x % cube_width;
    let face_y = y % cube_width;
    // off top
    if y == -1 {
        assert_eq!(direction, Direction::Up);
        return (
            (cube_width - face_x - 1) as usize,
            (cube_width) as usize,
            Direction::Down,
        );
    }

    if y == cube_width * 3 {
        // Off bottom mid
        if x < cube_width * 3 {
            assert_eq!(direction, Direction::Down);
            return (
                (cube_width - face_x - 1) as usize,
                ((cube_width * 2) - 1) as usize,
                Direction::Up,
            );
        } else {
            // off bottom right
            assert_eq!(direction, Direction::Down);
            return (0, (2 * cube_width - face_x - 1) as usize, Direction::Right);
        }
    }

    // off left side
    if x == -1 {
        assert_eq!(direction, Direction::Left);
        return (
            (cube_width * 4 - face_y - 1) as usize,
            (cube_width * 3 - 1) as usize,
            Direction::Up,
        );
    }

    // Off right side
    if x == cube_width * 4 {
        assert_eq!(direction, Direction::Right);
        return (
            (cube_width * 3 - 1) as usize,
            (cube_width - face_y - 1) as usize,
            Direction::Left,
        );
    }

    if x >= cube_width * 3 {
        if y < cube_width {
            // Off mid top right, end up bottom right
            assert_eq!(direction, Direction::Right);
            return (
                (cube_width * 4 - 1) as usize,
                (cube_width * 3 - face_y - 1) as usize,
                Direction::Left,
            );
        } else {
            match direction {
                // Off mid mid right, end up bottom right
                Direction::Right => {
                    return (
                        (cube_width * 4 - face_y - 1) as usize,
                        (cube_width * 2) as usize,
                        Direction::Down,
                    );
                }
                Direction::Up => {
                    return (
                        (cube_width * 3 - 1) as usize,
                        (cube_width * 2 - face_x - 1) as usize,
                        Direction::Left,
                    );
                }
                _ => panic!(
                    "Unexpected direction {} for coordinate ({}, {})",
                    direction, x, y
                ),
            }
        }
    }

    if x == cube_width * 2 - 1 {
        if y < cube_width {
            // Off mid top left, end up mid left
            if direction == Direction::Left {
                return (
                    (cube_width + face_y) as usize,
                    (cube_width) as usize,
                    Direction::Down,
                );
            }
        } else {
            // Off mid mid mid, end up bottom mid
            if direction == Direction::Left {
                return (
                    (cube_width * 2 - face_y - 1) as usize,
                    (cube_width * 2 - 1) as usize,
                    Direction::Up,
                );
            }
        }
    }

    if y == cube_width - 1 {
        if x < cube_width {
            // Off mid left top, end up top mid
            assert_eq!(direction, Direction::Up);
            return ((cube_width * 3 - face_x - 1) as usize, 0, Direction::Down);
        } else {
            assert_eq!(direction, Direction::Up);
            assert!(x >= cube_width && x < cube_width * 2);
            return ((2 * cube_width) as usize, face_x as usize, Direction::Right);
        }
    }

    if y == cube_width * 2 {
        if x < cube_width {
            assert_eq!(direction, Direction::Down);
            return (
                (cube_width * 3 - face_x - 1) as usize,
                (cube_width * 3 - 1) as usize,
                Direction::Up,
            );
        } else {
            assert_eq!(direction, Direction::Down);
            return (
                (cube_width * 2) as usize,
                (cube_width * 3 - face_x - 1) as usize,
                Direction::Right,
            );
        }
    }

    panic!(
        "Unexpected direction {} for coordinate ({}, {})",
        direction, x, y
    );
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let (mut grid, commands) = parse(lines);

        let mut current_col = grid.row(0).position(|t| *t == Tile::Open(None)).unwrap();
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
                            Tile::Open(_) => (next_col, next_row),
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
                                        Tile::Open(_) => {
                                            last_valid_position = (next_col, next_row);
                                            break;
                                        }
                                        Tile::Wall => {
                                            break;
                                        }
                                        Tile::Empty => (),
                                    }
                                }

                                last_valid_position
                            }
                        };
                        *grid.mut_at((current_col, current_row)) = Tile::Open(Some(current_dir));
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

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let (mut grid, commands) = parse(lines);

        let is_sample = grid.width == 16 && grid.height == 12;
        let face_length = num::integer::gcd(grid.width, grid.height);

        let mut current_col = grid.row(0).position(|t| *t == Tile::Open(None)).unwrap();
        let mut current_row = 0;
        let mut current_dir = Direction::Right;

        debug!("Starting at ({}, {})", current_col, current_row);

        *grid.mut_at((current_col, current_row)) = Tile::Open(Some(current_dir));
        debug!("Original grid {}", grid);

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
                        (current_col, current_row, current_dir) = if next_col >= 0
                            && next_col < grid.width as isize
                            && next_row >= 0
                            && next_row < grid.height as isize
                        {
                            let (next_col, next_row) = (next_col as usize, next_row as usize);
                            let next_tile = grid.at((next_col, next_row));
                            match next_tile {
                                Tile::Open(_) => (next_col, next_row, current_dir),
                                Tile::Wall => (current_col, current_row, current_dir),
                                Tile::Empty => {
                                    // Wrap around the cube
                                    let (next_col, next_row, next_dir) = wrap_around_cube(
                                        is_sample,
                                        face_length,
                                        (next_col as isize, next_row as isize),
                                        current_dir,
                                    );
                                    let next_tile = grid.at((next_col, next_row));
                                    debug!(
                                        "Wrapping around cube: ({}, {}) with direction {:?} to ({}, {}) with direction {:?}",
                                        next_col, next_row, current_dir, next_col, next_row, next_dir);
                                    if let Tile::Open(_) = next_tile {
                                        (next_col, next_row, next_dir)
                                    } else {
                                        (current_col, current_row, current_dir)
                                    }
                                }
                            }
                        } else {
                            let (next_col, next_row, next_dir) = wrap_around_cube(
                                is_sample,
                                face_length,
                                (next_col, next_row),
                                current_dir,
                            );
                            let next_tile = grid.at((next_col, next_row));
                            debug!(
                                        "Wrapping around cube: ({}, {}) with direction {:?} to ({}, {}) with direction {:?}",
                                        next_col, next_row, current_dir, next_col, next_row, next_dir);
                            if let Tile::Open(_) = next_tile {
                                (next_col, next_row, next_dir)
                            } else {
                                (current_col, current_row, current_dir)
                            }
                        };

                        *grid.mut_at((current_col, current_row)) = Tile::Open(Some(current_dir));
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
    let sample_3 = include_str!("../../samples/22_2.txt");
    let input = include_str!("../../inputs/22.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 6032),
        aoc::Input::new_sample(sample_2, 1038),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 5031),
        aoc::Input::new_sample(sample_2, 12056),
        aoc::Input::new_sample(sample_3, 1023),
        aoc::Input::new_final(input), // 93373 too low // 103134 //189097
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wrap_around_top_mid() {
        // Off top mid
        assert_eq!(
            wrap_around_cube(true, 4, (8, -1), Direction::Up),
            (3, 4, Direction::Down)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (10, -1), Direction::Up),
            (1, 4, Direction::Down)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (11, -1), Direction::Up),
            (0, 4, Direction::Down)
        );
    }

    #[test]
    fn test_wrap_around_bottom_mid() {
        // Off bottom mid
        assert_eq!(
            wrap_around_cube(true, 4, (8, 12), Direction::Down),
            (3, 7, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (10, 12), Direction::Down),
            (1, 7, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (11, 12), Direction::Down),
            (0, 7, Direction::Up)
        );
    }

    #[test]
    fn test_wrap_around_bottom_right() {
        // Off bottom right
        assert_eq!(
            wrap_around_cube(true, 4, (12, 12), Direction::Down),
            (0, 7, Direction::Right)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (15, 12), Direction::Down),
            (0, 4, Direction::Right)
        );
    }

    #[test]
    fn test_wrap_around_left() {
        // Off left
        assert_eq!(
            wrap_around_cube(true, 4, (-1, 4), Direction::Left),
            (15, 11, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (-1, 6), Direction::Left),
            (13, 11, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (-1, 7), Direction::Left),
            (12, 11, Direction::Up)
        );
    }

    #[test]
    fn test_wrap_around_right() {
        // Off right
        assert_eq!(
            wrap_around_cube(true, 4, (16, 8), Direction::Right),
            (11, 3, Direction::Left)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (16, 10), Direction::Right),
            (11, 1, Direction::Left)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (16, 11), Direction::Right),
            (11, 0, Direction::Left)
        );
    }

    #[test]
    fn test_wrap_around_mid_top_right() {
        assert_eq!(
            wrap_around_cube(true, 4, (12, 0), Direction::Right),
            (15, 11, Direction::Left)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (12, 3), Direction::Right),
            (15, 8, Direction::Left)
        );
    }

    #[test]
    fn test_wrap_around_mid_mid_right() {
        assert_eq!(
            wrap_around_cube(true, 4, (12, 4), Direction::Right),
            (15, 8, Direction::Down)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (12, 6), Direction::Right),
            (13, 8, Direction::Down)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (12, 7), Direction::Right),
            (12, 8, Direction::Down)
        );
    }

    #[test]
    fn test_wrap_around_bottom_right_up() {
        assert_eq!(
            wrap_around_cube(true, 4, (12, 7), Direction::Up),
            (11, 7, Direction::Left)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (15, 7), Direction::Up),
            (11, 4, Direction::Left)
        );
    }

    #[test]
    fn test_wrap_around_bottom_mid_left() {
        assert_eq!(
            wrap_around_cube(true, 4, (7, 8), Direction::Left),
            (7, 7, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (7, 10), Direction::Left),
            (5, 7, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (7, 11), Direction::Left),
            (4, 7, Direction::Up)
        );
    }
    #[test]
    fn test_wrap_around_top_left() {
        assert_eq!(
            wrap_around_cube(true, 4, (7, 0), Direction::Left),
            (4, 4, Direction::Down)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (7, 3), Direction::Left),
            (7, 4, Direction::Down)
        );
    }
    #[test]
    fn test_wrap_around_mid_left_up() {
        assert_eq!(
            wrap_around_cube(true, 4, (0, 3), Direction::Up),
            (11, 0, Direction::Down)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (3, 3), Direction::Up),
            (8, 0, Direction::Down)
        );
    }
    #[test]
    fn test_wrap_around_mid_mid_left_up() {
        assert_eq!(
            wrap_around_cube(true, 4, (4, 3), Direction::Up),
            (8, 0, Direction::Right)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (7, 3), Direction::Up),
            (8, 3, Direction::Right)
        );
    }

    #[test]
    fn test_wrap_around_mid_left_down() {
        assert_eq!(
            wrap_around_cube(true, 4, (0, 8), Direction::Down),
            (11, 11, Direction::Up)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (3, 8), Direction::Down),
            (8, 11, Direction::Up)
        );
    }

    #[test]
    fn test_wrap_around_mid_mid_left_down() {
        assert_eq!(
            wrap_around_cube(true, 4, (4, 8), Direction::Down),
            (8, 11, Direction::Right)
        );
        assert_eq!(
            wrap_around_cube(true, 4, (7, 8), Direction::Down),
            (8, 8, Direction::Right)
        );
    }
}
