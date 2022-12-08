use aoc::Solver;
use im::HashSet;
fn main() {
    let sample = include_str!("../../samples/3.txt");
    let input = include_str!("../../inputs/3.txt");

    let part_one_problems = [
        aoc::Input::new_sample(sample, 157),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 70),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}

fn find_common(line: &str) -> char {
    let (left_compartment, right_compartment) = line.split_at(line.len() / 2);
    let left_set = left_compartment.chars().collect::<HashSet<_>>();
    let right_set = right_compartment.chars().collect::<HashSet<_>>();
    let intersect = left_set.intersection(right_set);
    if intersect.iter().count() != 1 {
        println!(
            "Intersection between {} and {} is {:?}",
            left_compartment, right_compartment, intersect
        );
        panic!("Intersection is not 1");
    }

    *intersect.iter().next().unwrap()
}

fn compute_priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 'A' as usize + 27
    } else {
        c as usize - 'a' as usize + 1
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .map(|line| compute_priority(find_common(line)))
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        lines
            .chunks(3)
            .map(|group| {
                let common_item = group
                    .iter()
                    .map(|line| line.chars().collect::<HashSet<_>>())
                    .reduce(|a, b| a.intersection(b))
                    .expect("Should be one item");
                if common_item.len() != 1 {
                    println!("Not exactly 1 common item: {:?}", common_item);
                    panic!();
                }
                compute_priority(*common_item.iter().next().unwrap())
            })
            .sum::<usize>()
    }
}
