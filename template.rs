use itertools::Itertools;

fn part_one(lines: &[&str]) -> usize {
    0
}

fn part_two(lines: &[&str]) -> usize {
    0
}

fn main() {
    let sample = include_str!("../../samples/aaaaa.txt");
    let input = include_str!("../../inputs/aaaaa.txt");
    let part_one_problems = [
        aoc::Problem::new_sample(sample, 0), // TODO: Fill in expected sample result
        aoc::Problem::new_final(input),
    ];
    let part_one = aoc::Solution::new("part_one", &part_one, &part_one_problems);

    let part_two_problems = [
        aoc::Problem::new_sample(sample, 0), // TODO: Fill in expected sample result
        aoc::Problem::new_final(input),
    ];
    let part_two = aoc::Solution::new("part_two", &part_two, &part_two_problems);

    aoc::run_all(part_one, part_two);
}
