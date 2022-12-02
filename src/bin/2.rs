fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;

    enum Outcome {
        Win,
        Loss,
        Draw,
    }



    fn score_game(other: &str, me: &str) -> usize{
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

        selection_score + match outcome {
            Outcome::Win => 6 ,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }


    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        lines.iter().map(|line| {
            let (other, me) = line.split(' ').collect_tuple().unwrap();
            score_game(other, me)
        }).sum()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        let recalculated = lines.iter().map(|line| {
            let (other, outcome) = line.split(' ').collect_tuple().unwrap();
            let outcome = match outcome {
                "X" => Outcome::Loss,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => panic!("Invalid input"),
            };
            format!("{} {}", other, match (other, outcome) {
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
            })

        }).collect_vec();
        part_one(recalculated, _param)
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/2.txt"
        } else {
            "inputs/2.txt"
        }
    }

    fn get_lines(filename: &str) -> Vec<String> {
        let contents = fs::read_to_string(filename)
            .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
        contents.lines().map(|s| s.to_owned()).collect_vec()
    }

    const SAMPLE_PARAM: usize = 0;
    const REAL_PARAM: usize = 0;

    #[test]
    fn test_part_one_sample() {
        let result = part_one(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part one sample: {:?}", result);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 13009);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 0);
    }

}
