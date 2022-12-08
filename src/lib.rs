use itertools::Itertools;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub trait InputResult: Display + Sync + Eq + PartialEq + Debug {}
impl<T> InputResult for T where T: Display + Sync + Eq + PartialEq + Debug {}

pub trait Solver<D>: Sync
where
    D: InputResult,
{
    fn solve_part_one(&self, lines: &[&str]) -> D;
    fn solve_part_two(&self, lines: &[&str]) -> D;

    fn run_single(&self, solver: &(dyn Fn(&[&str]) -> D + Sync), lines: &[&str]) -> (D, Duration) {
        let start = Instant::now();
        let result = solver(lines);
        let elapsed = start.elapsed();
        (result, elapsed)
    }

    fn run_part_one(&self, lines: &[&str]) -> (D, Duration) {
        self.run_single(&|lines| self.solve_part_one(lines), lines)
    }

    fn run_all_for_solver<const PART: u8>(
        &self,
        solver: &(dyn Fn(&[&str]) -> D + Sync),
        inputs: &[Input<D>],
    ) {
        thread::scope(|s| {
            for (idx, input) in inputs.iter().enumerate() {
                s.spawn(move || {
                    let (result, elapsed) =
                        self.run_single(solver, get_lines(input.data).as_slice());
                    if let Some(solution) = &input.solution {
                        if solution == &result {
                            debug!(
                                "Part {PART} sample #{idx} passed: {} ({:?})",
                                result, elapsed
                            );
                        } else {
                            error!(
                                "Part {PART} sample #{idx} failed : {} (expected {}, {:?})",
                                result, solution, elapsed
                            );
                        }
                    } else {
                        info!("Part {PART} final: {} ({:?})", result, elapsed);
                    }
                });
            }
        })
    }

    fn run_all_part_one(&self, inputs: &[Input<D>]) {
        self.run_all_for_solver::<1>(&|lines| self.solve_part_one(lines), inputs);
    }

    fn run_part_two(&self, lines: &[&str]) -> (D, Duration) {
        self.run_single(&|lines| self.solve_part_two(lines), lines)
    }
    fn run_all_part_two(&self, inputs: &[Input<D>]) {
        self.run_all_for_solver::<2>(&|lines| self.solve_part_two(lines), inputs);
    }

    fn run(&self, part_one_inputs: &[Input<D>], part_two_inputs: &[Input<D>]) {
        SimpleLogger::new().env().init().unwrap();
        thread::scope(|s| {
            s.spawn(move || {
                self.run_all_part_one(part_one_inputs);
            });
            s.spawn(move || {
                self.run_all_part_two(part_two_inputs);
            });
        })
    }
}

pub struct Input<'a, D>
where
    D: InputResult,
{
    data: &'a str,
    solution: Option<D>,
}

impl<'a, D> Input<'a, D>
where
    D: InputResult,
{
    pub fn new_sample(sample: &'a str, solution: D) -> Self {
        Self {
            data: sample,
            solution: Some(solution),
        }
    }

    pub fn new_final(input: &'a str) -> Self {
        Self {
            data: input,
            solution: None,
        }
    }
}

fn get_lines(file: &str) -> Vec<&str> {
    file.lines().collect_vec()
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Grid<T> {
    pub state: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn new(state: Vec<T>, width: usize, height: usize) -> Grid<T> {
        Grid {
            state,
            width,
            height,
        }
    }

    pub fn from_2d(initial: Vec<Vec<T>>) -> Grid<T> {
        let height = initial.len();
        let width = initial[0].len();
        Grid {
            state: initial.into_iter().flatten().collect_vec(),
            width,
            height,
        }
    }

    pub fn pos_to_index(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;
        y * self.width + x
    }

    pub fn at(&self, pos: (usize, usize)) -> &T {
        &self.state[self.pos_to_index(pos)]
    }

    pub fn _mut_at(&mut self, pos: (usize, usize)) -> &mut T {
        let index = self.pos_to_index(pos);
        &mut self.state[index]
    }

    pub fn neighbors(&self, pos: (usize, usize)) -> impl Iterator<Item = &T> {
        let (x, y) = pos;
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let new_x = x as i64 + dx;
                    let new_y = y as i64 + dy;
                    if new_x >= 0
                        && new_x < self.width as i64
                        && new_y >= 0
                        && new_y < self.height as i64
                    {
                        Some(self.at((new_x as usize, new_y as usize)))
                    } else {
                        None
                    }
                }
            })
    }

    pub fn horizontal_neighbors(
        &self,
        pos: (usize, usize),
    ) -> (
        impl Iterator<Item = &T> + Clone,
        impl Iterator<Item = &T> + Clone,
    ) {
        let (x0, y0) = pos;
        let left_half = (0..x0).rev().map(move |x| self.at((x, y0)));
        let right_half = ((x0 + 1)..self.width).map(move |x| self.at((x, y0)));
        (left_half, right_half)
    }

    pub fn vertical_neighbors(
        &self,
        pos: (usize, usize),
    ) -> (
        impl Iterator<Item = &T> + Clone,
        impl Iterator<Item = &T> + Clone,
    ) {
        let (x0, y0) = pos;
        let top_half = (0..y0).rev().map(move |y| self.at((x0, y)));
        let bottom_half = ((y0 + 1)..self.height).map(move |y| self.at((x0, y)));
        (top_half, bottom_half)
    }

    pub fn neighbors_along_directions(
        &self,
        pos: (usize, usize),
    ) -> Vec<impl Iterator<Item = (usize, usize)>> {
        let (x, y) = pos;
        let (width, height) = (self.width, self.height);
        let delta = -1..=1;
        delta
            .clone()
            .cartesian_product(delta)
            .filter_map(move |(dx, dy)| {
                if dx == 0 && dy == 0 {
                    None
                } else {
                    let nums = 1..std::cmp::max(width, height);

                    // Have to make an in scope copy to appease borrow checker
                    let width = width;
                    let height = height;

                    Some(
                        nums.filter_map(move |d| {
                            let new_x = x as i64 + dx * d as i64;
                            let new_y = y as i64 + dy * d as i64;

                            if new_x >= 0 && new_y >= 0 {
                                Some((new_x as usize, new_y as usize))
                            } else {
                                None
                            }
                        })
                        .take_while(move |(new_x, new_y)| *new_x < width && *new_y < height),
                    )
                }
            })
            .collect_vec()
    }

    pub fn _to_2d(&self) -> Vec<Vec<&T>> {
        self.state
            .chunks(self.width)
            .map(|chunk| chunk.iter().collect_vec())
            .collect_vec()
    }

    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| (x, y))
    }

    pub fn from_lines(lines: &[&str], transformer: &dyn Fn(char) -> T) -> Grid<T> {
        let height = lines.len();
        let width = lines[0].len();
        let state = lines
            .iter()
            .flat_map(|line| line.chars().map(transformer))
            .collect_vec();
        Grid {
            state,
            width,
            height,
        }
    }
}

impl<T: Display> Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f)?;
        for chunk in self.state.chunks(self.width) {
            for t in chunk {
                write!(f, "{t}")?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}
