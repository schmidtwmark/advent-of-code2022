#![feature(anonymous_lifetime_in_impl_trait)]
use aoc::Grid;
use aoc::Solver;
use itertools::Itertools;
use log::{debug, info};

fn calculate_scenic_score(value: &u8, neighbors: impl Iterator<Item = &u8> + Clone) -> usize {
    let visible = neighbors.clone().take_while(|&n| n < value).count();
    if visible < neighbors.count() {
        visible + 1
    } else {
        visible
    }
}
struct Solution {}
impl Solver<usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let grid: Grid<u8> = Grid::from_lines(lines, &|c: char| c.to_digit(10).unwrap() as u8);
        debug!("Grid: {}, {}, {}", grid.width, grid.height, grid);
        grid.positions()
            .filter(|p| {
                let (left_neighbors, right_neighbors) = grid.horizontal_neighbors(*p);
                let (top_neighbors, bottom_neighbors) = grid.vertical_neighbors(*p);
                let val = grid.at(*p);
                let left_max = left_neighbors.max();
                let right_max = right_neighbors.max();
                let top_max = top_neighbors.max();
                let bottom_max = bottom_neighbors.max();

                [left_max, right_max, top_max, bottom_max]
                    .iter()
                    .any(|&max| if let Some(m) = max { val > m } else { true })
            })
            .count()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let grid: Grid<u8> = Grid::from_lines(lines, &|c: char| c.to_digit(10).unwrap() as u8);
        grid.positions()
            .map(|p| {
                let (left_neighbors, right_neighbors) = grid.horizontal_neighbors(p);
                let (top_neighbors, bottom_neighbors) = grid.vertical_neighbors(p);
                let val = grid.at(p);
                let left_score = calculate_scenic_score(val, left_neighbors);
                let right_score = calculate_scenic_score(val, right_neighbors);
                let top_score = calculate_scenic_score(val, top_neighbors);
                let bottom_score = calculate_scenic_score(val, bottom_neighbors);

                debug!(
                    "Value {val} at ({}, {}) -> [{}, {}, {}, {}]",
                    p.0, p.1, left_score, right_score, top_score, bottom_score
                );

                [left_score, right_score, top_score, bottom_score]
                    .iter()
                    .product::<usize>()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let sample = include_str!("../../samples/8.txt");
    let input = include_str!("../../inputs/8.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 21),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 8),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
