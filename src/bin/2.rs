use aoc::Solver;
use itertools::Itertools;
fn main() {
    let sample = include_str!("../../samples/2.txt");
    let input = include_str!("../../inputs/2.txt");

    let part_one_problems = [
        aoc::Input::new_sample(sample, 15),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 12),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

fn score_game(other: &str, me: &str) -> usize {
    let outcome = match (other, me) {
        ("A", "X") => Outcome::Draw,
        ("A", "Y") => Outcome::Win,
        ("A", "Z") => Outcome::Loss,
        ("B", "X") => Outcome::Loss,
        ("B", "Y") => Outcome::Draw,
        ("B", "Z") => Outcome::Win,
        ("C", "X") => Outcome::Win,
        ("C", "Y") => Outcome::Loss,
        ("C", "Z") => Outcome::Draw,
        _ => panic!("Invalid input"),
    };

    let selection_score = match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    selection_score
        + match outcome {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
}
struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        lines
            .iter()
            .map(|line| {
                let (other, me) = line.split(' ').collect_tuple().unwrap();
                score_game(other, me)
            })
            .sum()
    }
    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let recalculated = lines
            .iter()
            .map(|line| {
                let (other, outcome) = line.split(' ').collect_tuple().unwrap();
                let outcome = match outcome {
                    "X" => Outcome::Loss,
                    "Y" => Outcome::Draw,
                    "Z" => Outcome::Win,
                    _ => panic!("Invalid input"),
                };
                format!(
                    "{} {}",
                    other,
                    match (other, outcome) {
                        ("A", Outcome::Draw) => "X",
                        ("A", Outcome::Win) => "Y",
                        ("A", Outcome::Loss) => "Z",
                        ("B", Outcome::Loss) => "X",
                        ("B", Outcome::Draw) => "Y",
                        ("B", Outcome::Win) => "Z",
                        ("C", Outcome::Win) => "X",
                        ("C", Outcome::Loss) => "Y",
                        ("C", Outcome::Draw) => "Z",
                        _ => panic!("Invalid input"),
                    }
                )
            })
            .collect_vec();
        self.solve_part_one(
            recalculated
                .iter()
                .map(|s| s.as_str())
                .collect_vec()
                .as_slice(),
        )
    }
}
