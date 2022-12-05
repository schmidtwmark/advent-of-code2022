use aoc::Solver;
use itertools::Itertools;

struct PartOne {}
impl Solver<usize> for PartOne {
    const PART: u8 = 1;

    fn solve(&self, lines: &[&str]) -> usize {
        0
    }
}

struct PartTwo {}
impl Solver<usize> for PartTwo {
    const PART: u8 = 2;

    fn solve(&self, lines: &[&str]) -> usize {
        0
    }
}

fn main() {
    let sample = include_str!("../../samples/aaaaa.txt");
    let input = include_str!("../../inputs/aaaaa.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 0), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 0), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    PartOne {}.run(&part_one_problems);
    PartTwo {}.run(&part_two_problems);
}
