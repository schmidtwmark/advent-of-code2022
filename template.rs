use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/aaaaa.txt");
    let input = include_str!("../../inputs/aaaaa.txt");
    let part_one_problems = [
        aoc::Problem::new(sample, Some(0)),
        aoc::Problem::new(input, None),
    ];
    let part_one = aoc::Solution::new("part_one", &part_one, &part_one_problems);

    let part_two_problems = [
        aoc::Problem::new(sample, Some(0)),
        aoc::Problem::new(input, None),
    ];
    let part_two = aoc::Solution::new("part_two", &part_two, &part_two_problems);

    aoc::run_all(part_one, part_two);
}

fn part_one(lines: &[&str]) -> usize {
    0
}

fn part_two(lines: &[&str]) -> usize {
    0
}
