fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;

    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        0
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        0
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/aaaaa.txt"
        } else {
            "inputs/aaaaa.txt"
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
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 0);
    }

}
