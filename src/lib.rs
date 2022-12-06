use itertools::Itertools;
use log::{debug, error, info};
use simple_logger::SimpleLogger;
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
        SimpleLogger::new().init().unwrap();
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
