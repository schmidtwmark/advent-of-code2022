use std::str::FromStr;

use aoc::Solver;
use itertools::Itertools;
use log::debug;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(i64),
}

impl Packet {
    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => match a.compare(b) {
                            Ordering::Equal => continue,
                            other => return other,
                        },
                        (Some(_), None) => return Ordering::Greater,
                        (None, Some(_)) => return Ordering::Less,
                        (None, None) => return Ordering::Equal,
                    }
                }
            }
            (Packet::Integer(_), Packet::List(_)) => {
                let retry_packet = Packet::List(vec![self.clone()]);
                retry_packet.compare(&other)
            }
            (Packet::List(_), Packet::Integer(_)) => {
                let retry_packet = Packet::List(vec![other.clone()]);
                self.compare(&retry_packet)
            }
        }
    }

    fn from_str_internal(s: &str, start_idx: usize) -> (Packet, usize) {
        let mut list = Vec::new();
        let mut integer_string = String::new();

        let maybe_make_string = |lst: &mut Vec<Packet>, int_string: &mut String| {
            if !int_string.is_empty() {
                lst.push(Packet::Integer(int_string.parse::<i64>().unwrap()));
                int_string.clear();
            }
        };

        let mut idx = start_idx;
        while idx < s.len() {
            let char = s.chars().nth(idx).unwrap();
            match char {
                '[' => {
                    let (packet, index) = Self::from_str_internal(s, idx + 1);
                    idx = index;
                    list.push(packet);
                }
                ']' => {
                    maybe_make_string(&mut list, &mut integer_string);
                    break; // Exit this function level
                }
                '0'..='9' => {
                    integer_string.push(char);
                }
                ',' => {
                    maybe_make_string(&mut list, &mut integer_string);
                }
                _ => {
                    panic!("wat")
                }
            }
            idx += 1;
        }
        maybe_make_string(&mut list, &mut integer_string);

        (Packet::List(list), idx)
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s[1..s.len() - 1]; // trim front and end brackets
        let (packet, index) = Self::from_str_internal(s, 0);
        assert_eq!(index, s.len());
        Ok(packet)
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let packet_pairs = lines.split(|line| line.is_empty()).map(|pair| {
            pair.iter()
                .map(|line| line.parse::<Packet>().unwrap())
                .collect_tuple::<(Packet, Packet)>()
                .unwrap()
        });

        packet_pairs
            .enumerate()
            .filter_map(|(idx, (packet_a, packet_b))| {
                let ordering = packet_a.compare(&packet_b);
                debug!(
                    "{}: {:?} vs {:?} = {:?}",
                    idx + 1,
                    packet_a,
                    packet_b,
                    ordering
                );
                if ordering == Ordering::Less {
                    Some(idx + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/13.txt");
    let input = include_str!("../../inputs/13.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 13),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
