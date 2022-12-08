use std::collections::VecDeque;

use aoc::Solver;
use itertools::Itertools;

#[macro_use]
extern crate scan_fmt;

type Stack = VecDeque<char>;

struct Instructon {
    count: usize,
    from: usize,
    to: usize,
}

impl Instructon {
    fn apply(&self, stacks: &mut [Stack]) {
        let from_stack = &mut stacks[self.from];
        let mut temp_stack = Stack::new();

        for _ in 0..self.count {
            temp_stack.push_back(from_stack.pop_front().unwrap());
        }

        let to_stack = &mut stacks[self.to];
        temp_stack.iter().for_each(|c| to_stack.push_front(*c));
    }

    fn apply_advanced(&self, stacks: &mut [Stack]) {
        let from_stack = &mut stacks[self.from];
        let mut temp_stack = Stack::new();

        for _ in 0..self.count {
            temp_stack.push_front(from_stack.pop_front().unwrap());
        }

        let to_stack = &mut stacks[self.to];
        temp_stack.iter().for_each(|c| to_stack.push_front(*c));
    }
}

fn read_stack_at_index(lines: &[&str], index: usize) -> Stack {
    lines
        .iter()
        .filter_map(|line| {
            let c = line.chars().nth(index).unwrap();
            if c != ' ' {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}

fn read_instructions(lines: &[&str]) -> Vec<Instructon> {
    lines
        .iter()
        .map(|line| {
            let (count, from, to) =
                scan_fmt!(line, "move {} from {} to {}", usize, usize, usize).unwrap();
            Instructon {
                count,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect()
}

fn read_input(lines: &[&str]) -> (Vec<Stack>, Vec<Instructon>) {
    let (stack_text, instructions_text) =
        lines.split(|line| line.is_empty()).collect_tuple().unwrap();
    let stack_text = &stack_text[0..stack_text.len() - 1]; // Drop numbered columns
    let width = stack_text.first().unwrap().len();

    let stacks = (0..width)
        .skip(1)
        .step_by(4)
        .map(|idx| read_stack_at_index(stack_text, idx))
        .collect_vec();

    let instructions = read_instructions(instructions_text);
    (stacks, instructions)
}

struct Solution {}
impl Solver<'_, String> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> String {
        let (mut stacks, instructions) = read_input(lines);

        for instruction in instructions {
            instruction.apply(&mut stacks);
        }

        stacks.iter().filter_map(|stack| stack.front()).collect()
    }
    fn solve_part_two(&self, lines: &[&str]) -> String {
        let (mut stacks, instructions) = read_input(lines);

        for instruction in instructions {
            instruction.apply_advanced(&mut stacks);
        }

        stacks.iter().filter_map(|stack| stack.front()).collect()
    }
}

fn main() {
    let sample = include_str!("../../samples/5.txt");
    let input = include_str!("../../inputs/5.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, "CMZ".to_owned()),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, "MCD".to_owned()),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
