use itertools::Itertools;
use std::thread;
use std::time::Instant;

// pub type Solver = dyn Fn(&[String]) -> usize;

pub struct ProblemSolution<'a> {
    solver: &'a (dyn Fn(&[&str]) -> usize + Sync),
    sample_solution: usize,
}

impl<'a> ProblemSolution<'a> {
    pub fn new(solver: &'a (dyn Fn(&[&str]) -> usize + Sync), sample_solution: usize) -> Self {
        Self {
            solver,
            sample_solution,
        }
    }

    pub fn run(&self, input: &[&str]) -> usize {
        (self.solver)(input)
    }
}

fn get_lines(file: &str) -> Vec<&str> {
    file.lines().collect_vec()
}

pub fn run_all(part_one: ProblemSolution, part_two: ProblemSolution, sample: &str, input: &str) {
    let sample = get_lines(sample);
    let real = get_lines(input);

    thread::scope(|s| {
        s.spawn(|| {
            let result = part_one.run(&sample);
            assert_eq!(result, part_one.sample_solution);
        });
        s.spawn(|| {
            let start = Instant::now();
            let result = part_one.run(&real);
            println!("Part one: {:?}, took {:?}", result, start.elapsed());
        });
        s.spawn(|| {
            let result = part_two.run(&sample);
            assert_eq!(result, part_two.sample_solution);
        });
        s.spawn(|| {
            let start = Instant::now();
            let result = part_two.run(&real);
            println!("Part two: {:?}, took {:?}", result, start.elapsed());
        });
    });
}
