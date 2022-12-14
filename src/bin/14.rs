use aoc::Solver;
use im::HashSet;
use itertools::Itertools;
use log::debug;
use std::str::FromStr;

type Position = (usize, usize);

#[derive(Debug)]
struct Rock {
    structure: HashSet<Position>,
    lowest_height: usize,
}

impl FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lowest_height = 0;
        let structure = s
            .split(" -> ")
            .map(|structure| {
                let (x, y) = structure
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();
                if y > lowest_height {
                    lowest_height = y;
                }
                (x, y)
            })
            .tuple_windows()
            .flat_map(|((x1, y1), (x2, y2))| {
                debug!("Start {:?} End {:?}", (x1, y1), (x2, y2));
                let (minx, maxx) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let (miny, maxy) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                let vs = (minx..=maxx).flat_map(move |x| (miny..=maxy).map(move |y| (x, y)));
                debug!("{:?}", vs.clone().collect_vec());
                vs
            })
            .collect();
        Ok(Rock {
            structure,
            lowest_height,
        })
    }
}

fn get_total_structure(lines: &[&str]) -> Rock {
    let rocks = lines.iter().map(|line| Rock::from_str(line).unwrap());
    debug!("{:?}", rocks.clone().collect_vec());

    let combined = rocks
        .reduce(|a, b| Rock {
            structure: a.structure.union(b.structure),
            lowest_height: a.lowest_height.max(b.lowest_height),
        })
        .unwrap();
    debug!("Combined: {:?}", combined);
    combined
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let combined = get_total_structure(lines);
        let start_pos = (500, 0);
        let mut sand_pos = start_pos;
        let mut sand_positions = std::collections::HashSet::new();

        loop {
            let (sand_x, sand_y) = sand_pos;
            let below = (sand_x, sand_y + 1);
            debug!("Sand: {:?}", sand_pos);

            if sand_y > combined.lowest_height {
                // Sand will always fall
                break;
            }
            sand_pos = if combined.structure.contains(&below) || sand_positions.contains(&below) {
                // Sand is blocked, time to check if it can flow left or right
                let left = (sand_x - 1, sand_y + 1);
                let right = (sand_x + 1, sand_y + 1);
                if combined.structure.contains(&left) || sand_positions.contains(&left) {
                    // Sand is blocked on the left, check if it can flow right
                    if combined.structure.contains(&right) || sand_positions.contains(&right) {
                        // Sand is blocked on the right, rest and reset
                        sand_positions.insert(sand_pos);
                        start_pos
                    } else {
                        // Sand can flow right
                        right
                    }
                } else {
                    // Sand can flow left
                    left
                }
            } else {
                below
            }
        }

        sand_positions.len()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let combined = get_total_structure(lines);
        let start_pos = (500, 0);
        let mut sand_pos = start_pos;
        let mut sand_positions = std::collections::HashSet::new();

        let infinite_wall_height = combined.lowest_height + 2;

        loop {
            let (sand_x, sand_y) = sand_pos;
            let below = (sand_x, sand_y + 1);
            debug!("Sand: {:?}", sand_pos);

            if sand_positions.contains(&start_pos) {
                // Sand can no longer fall
                break;
            }
            sand_pos = if combined.structure.contains(&below)
                || sand_positions.contains(&below)
                || below.1 >= infinite_wall_height
            {
                // Sand is blocked, time to check if it can flow left or right
                let left = (sand_x - 1, sand_y + 1);
                let right = (sand_x + 1, sand_y + 1);
                if combined.structure.contains(&left)
                    || sand_positions.contains(&left)
                    || left.1 >= infinite_wall_height
                {
                    // Sand is blocked on the left, check if it can flow right
                    if combined.structure.contains(&right)
                        || sand_positions.contains(&right)
                        || right.1 >= infinite_wall_height
                    {
                        // Sand is blocked on the right, rest and reset
                        sand_positions.insert(sand_pos);
                        start_pos
                    } else {
                        // Sand can flow right
                        right
                    }
                } else {
                    // Sand can flow left
                    left
                }
            } else {
                below
            }
        }

        sand_positions.len()
    }
}

fn main() {
    let sample = include_str!("../../samples/14.txt");
    let input = include_str!("../../inputs/14.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 24),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 93),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
