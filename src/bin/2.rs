use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/2.txt");
    let input = include_str!("../../inputs/2.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 15);
    let part_two = aoc::ProblemSolution::new(&part_two, 12);
    aoc::run_all(part_one, part_two, sample, input);
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

fn part_one(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| {
            let (other, me) = line.split(' ').collect_tuple().unwrap();
            score_game(other, me)
        })
        .sum()
}

fn part_two(lines: &[String]) -> usize {
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
    part_one(&recalculated)
}
