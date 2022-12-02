use itertools::Itertools;

fn main() {
    aoc::run_all(part_one, part_two, 24000, 45000);
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
