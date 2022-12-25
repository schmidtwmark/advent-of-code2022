use std::{fmt::write, str::FromStr};

use aoc::Solver;
use itertools::Itertools;
use log::debug;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SnafuNumber {
    digits: Vec<i8>,
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut seen_nonzero = false;
        for digit in &self.digits {
            match digit {
                1..=2 => write!(f, "{digit}")?,
                0 => {
                    if seen_nonzero {
                        write!(f, "0")?
                    }
                }
                -1 => write!(f, "-")?,
                -2 => write!(f, "=")?,
                _ => panic!("Invalid digit {} in number {:?}", digit, self),
            }
            if digit != &0 {
                seen_nonzero = true;
            }
        }
        Ok(())
    }
}

impl FromStr for SnafuNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .rev()
            .map(|c| match c {
                '0'..='2' => c.to_digit(10).unwrap() as i8,
                '-' => -1,
                '=' => -2,
                _ => panic!("Invalid digit {} in number {}", c, s),
            })
            .collect_vec();
        Ok(SnafuNumber { digits })
    }
}
impl SnafuNumber {
    fn from_decimal(decimal: i128) -> Self {
        if decimal == 0 {
            return SnafuNumber { digits: vec![0] };
        }
        let mut digits = Vec::new();
        let mut mut_decimal = decimal;

        let places_needed = (((decimal as f64).log10() + 1.0) as u32) * 2;
        debug!("{} needs {} places", decimal, places_needed);

        for place in (0..places_needed).rev() {
            let place_value = 5_i128.pow(place);

            for val in -2..=2 {
                let test = mut_decimal - val * place_value;
                let max_remaining = SnafuNumber {
                    digits: (0..place).map(|_| 2).collect_vec(),
                }
                .to_decimal()
                .abs();

                debug!(
                    "{} - {} * {} = {}, max_remaining= {}",
                    mut_decimal, val, place_value, test, max_remaining
                );

                if test.abs() <= max_remaining {
                    digits.push(val as i8);
                    mut_decimal -= val * place_value;
                    break;
                }
            }
        }

        let out = SnafuNumber { digits };
        debug!("{} -> {} with raw: ({:?})", decimal, out, out);
        // assert_eq!(decimal, out.to_decimal());
        out
    }

    fn to_decimal(&self) -> i128 {
        let mut decimal = 0;
        for (i, digit) in self.digits.iter().enumerate() {
            let place = 5_i128.pow(i as u32);
            decimal += place * (*digit as i128);
        }
        decimal
    }
}

struct Solution {}
impl Solver<'_, String> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> String {
        let nums = lines
            .iter()
            .map(|line| SnafuNumber::from_str(line).unwrap())
            .collect_vec();

        let sum = nums.iter().map(|num| num.to_decimal()).sum::<i128>();

        let snafu_sum = SnafuNumber::from_decimal(sum);
        format!("{snafu_sum}")
    }

    fn solve_part_two(&self, _lines: &[&str]) -> String {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/25.txt");
    let input = include_str!("../../inputs/25.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, "2=-1=0".to_owned()),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            SnafuNumber::from_str("1").unwrap(),
            SnafuNumber { digits: vec![1] }
        );
        assert_eq!(
            SnafuNumber::from_str("1=").unwrap(),
            SnafuNumber {
                digits: vec![-2, 1]
            }
        );
        assert_eq!(
            SnafuNumber::from_str("1121-1110-1=0").unwrap(),
            SnafuNumber {
                digits: vec![0, -2, 1, -1, 0, 1, 1, 1, -1, 1, 2, 1, 1]
            }
        );
    }
}
