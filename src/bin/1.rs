fn main() {
    println!("Run unit tests with cargo test");
}

#[cfg(test)]
mod tests {

    use itertools::Itertools;
    use std::fs;

    fn part_one(lines: Vec<String>, _param: usize) -> usize {
        lines.split(|line| line.is_empty()).map(|group| {
            // Convert each string to int and sum
           group.iter().map(|x| x.parse::<usize>().unwrap()).sum::<usize>() 
        }).max().unwrap()
    }

    fn part_two(lines: Vec<String>, _param: usize) -> usize {
        let sums = lines.split(|line| line.is_empty()).map(|group| {
            // Convert each string to int and sum
           group.iter().map(|x| x.parse::<usize>().unwrap()).sum::<usize>() 
        }).collect_vec();
        let get_top = 3;
        // Get 3 largest values from sums
        sums.iter().sorted().rev().take(get_top).sum()
        
    }

    fn get_filename(sample: bool) -> &'static str {
        if sample {
            "samples/1.txt"
        } else {
            "inputs/1.txt"
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
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part one real: {:?}", result);
        assert_eq!(result, 66306);
    }

    #[test]
    fn test_part_two_sample() {
        let result = part_two(get_lines(get_filename(true)), SAMPLE_PARAM);
        println!("Part two sample: {:?}", result);
        assert_eq!(result, 45000);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(get_lines(get_filename(false)), REAL_PARAM);
        println!("Part two real: {:?}", result);
        assert_eq!(result, 195292);
    }

}
