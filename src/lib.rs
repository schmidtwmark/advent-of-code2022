use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Display;
use std::thread;
use std::time::Instant;

pub trait InputResult: Display + Sync + Eq + PartialEq + Debug {}
impl<T> InputResult for T where T: Display + Sync + Eq + PartialEq + Debug {}

pub trait Solver<D>: Sync
where
    D: InputResult,
{
    const PART: u8;
    fn solve(&self, lines: &[&str]) -> D;

    fn run(&self, inputs: &[Input<D>]) {
        thread::scope(|s| {
            for input in inputs {
                s.spawn(move || {
                    let start = Instant::now();
                    let result = self.solve(get_lines(input.data).as_slice());
                    let elapsed = start.elapsed();
                    if let Some(solution) = &input.solution {
                        assert_eq!(result, *solution);
                    } else {
                        println!("Part {}: {} ({:?})", Self::PART, result, elapsed);
                    }
                });
            }
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
