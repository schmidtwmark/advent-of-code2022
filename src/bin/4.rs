use std::convert::TryFrom;

use aoc::Solver;
use itertools::Itertools;

struct SectionAssignment {
    min: usize,
    max: usize,
}

impl TryFrom<&str> for SectionAssignment {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (min, max) = value
            .split('-')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { min, max })
    }
}

impl SectionAssignment {
    fn contains_other(&self, section: &SectionAssignment) -> bool {
        self.min <= section.min && self.max >= section.max
    }

    fn overlaps(&self, section: &SectionAssignment) -> bool {
        self.min <= section.max && section.min <= self.max
    }
}

fn to_sections<'a>(
    lines: &'a [&str],
) -> impl Iterator<Item = (SectionAssignment, SectionAssignment)> + 'a {
    lines.iter().map(|line| {
        let (a, b) = line
            .split(',')
            .map(|line| SectionAssignment::try_from(line).expect("Invalid input"))
            .collect_tuple()
            .unwrap();
        (a, b)
    })
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        to_sections(lines)
            .filter(|(a, b)| a.contains_other(b) || b.contains_other(a))
            .count()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        to_sections(lines).filter(|(a, b)| a.overlaps(b)).count()
    }
}

fn main() {
    let sample = include_str!("../../samples/4.txt");
    let input = include_str!("../../inputs/4.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 2),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 4),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
