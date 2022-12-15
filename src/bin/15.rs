use aoc::Solver;
use im::HashMap;
use itertools::Itertools;
use log::{debug, info};
use std::{collections::HashSet, time::Instant};
#[macro_use]
extern crate scan_fmt;

type Sensor = (i64, i64);
type Beacon = (i64, i64);

fn read_line(line: &str) -> (Sensor, Beacon) {
    if let Ok((sx, sy, bx, by)) = scan_fmt!(
        line.trim(),
        "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
        i64,
        i64,
        i64,
        i64
    ) {
        ((sx, sy), (bx, by))
    } else {
        panic!("Failed to parse line {}", line);
    }
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let target = if lines.len() == 14 { 10 } else { 2000000 };
        let sensors: HashMap<Sensor, Beacon> = lines.iter().map(|line| read_line(line)).collect();
        let beacons: HashSet<Beacon> = sensors.values().cloned().collect();

        let mut seen_xs: std::collections::HashSet<i64> = std::collections::HashSet::new();
        for (sensor, beacon) in sensors.into_iter() {
            let (sx, sy) = sensor;
            let (bx, by) = beacon;
            let distance_to_line = target - sy;
            let manhattan_distance = (bx - sx).abs() + (by - sy).abs();
            debug!(
                "Sensor: {:?}, Beacon: {:?}, Manhattan Distance: {}, Distance to Line: {}",
                sensor, beacon, manhattan_distance, distance_to_line
            );
            if distance_to_line.abs() <= manhattan_distance {
                // at sy, field is m_d wide
                // at sy +-1 field is m_d - 2 wide
                let width = 2 * (manhattan_distance - (distance_to_line.abs()));
                let start_x = sx - width / 2;
                let end_x = sx + width / 2;
                debug!(
                    "Target line is occluded by this beacon from {} to {}",
                    start_x, end_x
                );
                (start_x..=end_x).for_each(|x| {
                    seen_xs.insert(x);
                })
            }
        }

        debug!("Seen xs: {:?}", seen_xs);

        seen_xs
            .into_iter()
            .filter(|x| !beacons.contains(&(*x, target)))
            .count()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let target = if lines.len() == 14 { 20 } else { 4000000 };
        let sensors: HashMap<Sensor, (Beacon, i64)> = lines
            .iter()
            .map(|line| {
                let (sensor, beacon) = read_line(line);
                let distance = manhattan_distance(sensor, beacon);
                (sensor, (beacon, distance))
            })
            .collect();
        let mut x = 0;
        while x <= target {
            let mut y = 0;
            let start = Instant::now();
            while y <= target {
                debug!("Checking target {:?}", (x, y));
                let mut occluded = false;
                for (sensor, (beacon, distance)) in sensors.iter() {
                    let target_distance = manhattan_distance((x, y), *sensor);
                    if target_distance <= *distance {
                        debug!(
                            "Target {:?} is occluded by sensor {:?} and beacon {:?}",
                            (x, y),
                            sensor,
                            beacon
                        );
                        let distance_to_y = (y - sensor.1).abs();
                        // TODO: Move to the next possible y value
                        let occluded_width = distance - distance_to_y;

                        occluded = true;
                        break;
                    }
                }
                if !occluded {
                    info!("Target {:?} is not occluded by any beacon!", (x, y));
                    return (x * 4000000 + y) as usize;
                }
                y += 1;
            }
            info!("Completed checking row: {} in {:?}", x, start.elapsed());
            x += 1;
        }
        0
    }
}

fn main() {
    let sample = include_str!("../../samples/15.txt");
    let input = include_str!("../../inputs/15.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 26),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 56000011),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
