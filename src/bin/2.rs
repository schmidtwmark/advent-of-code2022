use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/2.txt");
    let input = include_str!("../../inputs/2.txt");

    let part_one_problems = [
        aoc::Problem::new_sample(sample, 15),
        aoc::Problem::new_final(input),
    ];
    let part_one = aoc::Solution::new("part_one", &part_one, &part_one_problems);

    let part_two_problems = [
        aoc::Problem::new_sample(sample, 12),
        aoc::Problem::new_final(input),
    ];
    let part_two = aoc::Solution::new("part_two", &part_two, &part_two_problems);

    aoc::run_all(part_one, part_two);
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

fn part_one(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| {
            let (other, me) = line.split(' ').collect_tuple().unwrap();
            score_game(other, me)
        })
        .sum()
}

fn part_two(lines: &[&str]) -> usize {
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
    part_one(
        recalculated
            .iter()
            .map(|s| s.as_str())
            .collect_vec()
            .as_slice(),
    )
}
