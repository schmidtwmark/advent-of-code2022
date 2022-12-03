use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/1.txt");
    let input = include_str!("../../inputs/1.txt");

    let part_one_problems = [
        aoc::Problem::new(sample, Some(24000)),
        aoc::Problem::new(input, None),
    ];
    let part_one = aoc::Solution::new("part_one", &part_one, &part_one_problems);

    let part_two_problems = [
        aoc::Problem::new(sample, Some(45000)),
        aoc::Problem::new(input, None),
    ];
    let part_two = aoc::Solution::new("part_two", &part_two, &part_two_problems);

    aoc::run_all(part_one, part_two);
}

fn part_one(lines: &[&str]) -> usize {
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

fn part_two(lines: &[&str]) -> usize {
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
