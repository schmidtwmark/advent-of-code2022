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

    Solution {}.run(&part_one_problems, &part_two_problems);
}

struct Solution {}
impl Solver<usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
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

    fn solve_part_two(&self, lines: &[&str]) -> usize {
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
