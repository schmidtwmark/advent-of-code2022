use aoc::Grid;
use aoc::Solver;
use itertools::Itertools;
use log::debug;
use log::info;
use std::{collections::VecDeque, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let op = words.next().unwrap();
        match op {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::Addx(words.next().unwrap().parse::<isize>().unwrap())),
            _ => panic!("Unknown instruction: {}", s),
        }
    }
}

impl Instruction {
    fn cycles(&self) -> isize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

fn get_strengths(instructions: impl Iterator<Item = Instruction>) -> VecDeque<isize> {
    let mut x = 1;

    let signal_strengths: VecDeque<_> = instructions
        .flat_map(|i| {
            let cycles = i.cycles();
            let out = vec![x; cycles as usize];

            if let Instruction::Addx(val) = i {
                x += val;
            }

            debug!("{:?}: -> {:?}", i, out);
            out
        })
        .collect();

    signal_strengths
}

struct Solution {}
impl Solver<'_, isize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> isize {
        let instructions = lines.iter().map(|s| Instruction::from_str(s).unwrap());

        let mut signal_strengths = get_strengths(instructions);
        signal_strengths.push_front(0); // Offset
        let signal_strengths = signal_strengths.into_iter().enumerate().collect_vec();
        debug!("Signals: {:?}", signal_strengths);

        let filtered = signal_strengths.iter().skip(20).step_by(40).collect_vec();
        debug!("Filtered: {:?}", filtered);

        filtered.iter().map(|(idx, x)| *idx as isize * x).sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> isize {
        let instructions = lines.iter().map(|s| Instruction::from_str(s).unwrap());

        let signal_strengths = get_strengths(instructions);

        let width = 40;
        let height = 6;
        let mut grid = Grid::new(vec![' '; width * height], 40, 6);

        for (cycle, x) in signal_strengths.iter().enumerate() {
            let diff = (x - (cycle % 40) as isize).abs();

            *grid.mut_at(grid.index_to_pos(cycle)) = if diff <= 1 { '#' } else { '.' };

            debug!("Cycle {}, x is {}:\n{}, ", cycle, x, grid);
        }

        info!("Final Grid: {}", grid);
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/10.txt");
    let sample_2 = include_str!("../../samples/10_2.txt");
    let input = include_str!("../../inputs/10.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 13140),
        aoc::Input::new_sample(sample_2, 0),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
