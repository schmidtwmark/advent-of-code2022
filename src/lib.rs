use itertools::Itertools;
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

    fn run_single(&self, solver: &dyn Fn(&[&str]) -> D, lines: &[&str]) -> (D, Duration) {
        let start = Instant::now();
        let result = solver(lines);
        let elapsed = start.elapsed();
        (result, elapsed)
    }

    fn run_part_one(&self, lines: &[&str]) -> (D, Duration) {
        self.run_single(&|lines| self.solve_part_one(lines), lines)
    }

    fn run_all_part_one(&self, inputs: &[Input<D>]) {
        thread::scope(|s| {
            for input in inputs {
                s.spawn(move || {
                    let (result, elapsed) = self.run_part_one(get_lines(input.data).as_slice());
                    if let Some(solution) = &input.solution {
                        assert_eq!(result, *solution);
                    } else {
                        println!("Part 1: {} ({:?})", result, elapsed);
                    }
                });
            }
        })
    }

    fn run_part_two(&self, lines: &[&str]) -> (D, Duration) {
        self.run_single(&|lines| self.solve_part_two(lines), lines)
    }
    fn run_all_part_two(&self, inputs: &[Input<D>]) {
        thread::scope(|s| {
            for input in inputs {
                s.spawn(move || {
                    let (result, elapsed) = self.run_part_two(get_lines(input.data).as_slice());
                    if let Some(solution) = &input.solution {
                        assert_eq!(result, *solution);
                    } else {
                        println!("Part 2: {} ({:?})", result, elapsed);
                    }
                });
            }
        })
    }

    fn run(&self, part_one_inputs: &[Input<D>], part_two_inputs: &[Input<D>]) {
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
