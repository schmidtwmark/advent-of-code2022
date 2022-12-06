use aoc::Solver;
use im::HashSet;
use itertools::Itertools;

fn find_first_unique_packet(line: &str, unique_count: usize) -> usize {
    line.chars()
        .enumerate()
        .collect_vec()
        .windows(unique_count)
        .filter_map(|window| {
            let idx = window[0].0;
            if window
                .iter()
                .map(|(_, c)| c)
                .collect::<HashSet<&char>>()
                .len()
                == unique_count
            {
                Some(idx + unique_count)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

struct Solution {}
impl Solver<usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let line = lines.first().unwrap();
        find_first_unique_packet(line, 4)
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let line = lines.first().unwrap();
        find_first_unique_packet(line, 14)
    }
}

fn main() {
    let sample = include_str!("../../samples/6.txt");
    let input = include_str!("../../inputs/6.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 7),
        aoc::Input::new_sample("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        aoc::Input::new_sample("nppdvjthqldpwncqszvftbrmjlhg", 6),
        aoc::Input::new_sample("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        aoc::Input::new_sample("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 19),
        aoc::Input::new_sample("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        aoc::Input::new_sample("nppdvjthqldpwncqszvftbrmjlhg", 23),
        aoc::Input::new_sample("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        aoc::Input::new_sample("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
