use std::convert::TryFrom;

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

fn part_one(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter_map(|line| {
            let (a, b) = line
                .split(',')
                .map(|line| SectionAssignment::try_from(line).expect("Invalid input"))
                .collect_tuple()
                .unwrap();

            if a.contains_other(&b) || b.contains_other(&a) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn part_two(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter_map(|line| {
            let (a, b) = line
                .split(',')
                .map(|line| SectionAssignment::try_from(line).expect("Invalid input"))
                .collect_tuple()
                .unwrap();

            if a.overlaps(&b) || b.overlaps(&a) {
                Some(())
            } else {
                None
            }
        })
        .count()
}

fn main() {
    let sample = include_str!("../../samples/4.txt");
    let input = include_str!("../../inputs/4.txt");
    let part_one_problems = [
        aoc::Problem::new_sample(sample, 2),
        aoc::Problem::new_final(input),
    ];
    let part_one = aoc::Solution::new("part_one", &part_one, &part_one_problems);

    let part_two_problems = [
        aoc::Problem::new_sample(sample, 4),
        aoc::Problem::new_final(input),
    ];
    let part_two = aoc::Solution::new("part_two", &part_two, &part_two_problems);

    aoc::run_all(&[part_one, part_two]);
}
