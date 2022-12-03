use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Display;
use std::thread;
use std::time::Instant;

pub trait ProblemResult: Display + Sync + Eq + PartialEq + Debug {}
impl<T> ProblemResult for T where T: Display + Sync + Eq + PartialEq + Debug {}

pub struct Problem<'a, D>
where
    D: ProblemResult,
{
    input: &'a str,
    solution: Option<D>,
}

impl<'a, D> Problem<'a, D>
where
    D: ProblemResult,
{
    pub fn new(input: &'a str, solution: Option<D>) -> Self {
        Self { input, solution }
    }
}

pub struct Solution<'a, D>
where
    D: ProblemResult,
{
    name: &'a str,
    solver: &'a (dyn Fn(&[&str]) -> D + Sync),
    problems: &'a [Problem<'a, D>],
}

impl<'a, D> Solution<'a, D>
where
    D: ProblemResult,
{
    pub fn new(
        name: &'a str,
        solver: &'a (dyn Fn(&[&str]) -> D + Sync),
        problems: &'a [Problem<'a, D>],
    ) -> Self {
        Self {
            name,
            solver,
            problems,
        }
    }

    pub fn run_all(&self) {
        thread::scope(|s| {
            for problem in self.problems {
                s.spawn(move || {
                    let start = Instant::now();
                    let result = (self.solver)(get_lines(problem.input).as_slice());
                    let elapsed = start.elapsed();
                    if let Some(solution) = &problem.solution {
                        assert_eq!(result, *solution);
                    } else {
                        println!("{}: {} ({:?})", self.name, result, elapsed);
                    }
                });
            }
        })
    }
}

fn get_lines(file: &str) -> Vec<&str> {
    file.lines().collect_vec()
}

pub fn run_all<D>(part_one: Solution<D>, part_two: Solution<D>)
where
    D: ProblemResult,
{
    thread::scope(|s| {
        s.spawn(|| {
            part_one.run_all();
        });
        s.spawn(|| {
            part_two.run_all();
        });
    });
}
