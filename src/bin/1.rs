use aoc::Solver;
use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/1.txt");
    let input = include_str!("../../inputs/1.txt");

    let part_one_problems = [
        aoc::Input::new_sample(sample, 24000),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 45000),
        aoc::Input::new_final(input),
    ];

    PartOne {}.run(&part_one_problems);
    PartTwo {}.run(&part_two_problems);
}

struct PartOne {}
impl Solver<usize> for PartOne {
    const PART: u8 = 1;

    fn solve(&self, lines: &[&str]) -> usize {
        lines
            .split(|line| line.is_empty())
            .map(|group| {
                // Convert each string to int and sum
                group
                    .iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .max()
            .unwrap()
    }
}

struct PartTwo {}
impl aoc::Solver<usize> for PartTwo {
    const PART: u8 = 2;

    fn solve(&self, lines: &[&str]) -> usize {
        let sums = lines
            .split(|line| line.is_empty())
            .map(|group| {
                // Convert each string to int and sum
                group
                    .iter()
                    .map(|x| x.parse::<usize>().unwrap())
                    .sum::<usize>()
            })
            .collect_vec();
        let get_top = 3;
        // Get 3 largest values from sums
        sums.iter().sorted().rev().take(get_top).sum()
    }
}
