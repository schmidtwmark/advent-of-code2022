use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/1.txt");
    let input = include_str!("../../inputs/1.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 24000);
    let part_two = aoc::ProblemSolution::new(&part_two, 45000);
    aoc::run_all(part_one, part_two, sample, input);
}

fn part_one(lines: &[String]) -> usize {
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

fn part_two(lines: &[String]) -> usize {
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
