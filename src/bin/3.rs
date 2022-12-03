use im::HashSet;
fn main() {
    let sample = include_str!("../../samples/3.txt");
    let input = include_str!("../../inputs/3.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 157);
    let part_two = aoc::ProblemSolution::new(&part_two, 70);
    aoc::run_all(part_one, part_two, sample, input);
}

fn find_common(line: &str) -> char {
    let (left_compartment, right_compartment) = line.split_at(line.len() / 2);
    let left_set = left_compartment.chars().collect::<HashSet<_>>();
    let right_set = right_compartment.chars().collect::<HashSet<_>>();
    let intersect = left_set.intersection(right_set);
    if intersect.iter().count() != 1 {
        println!(
            "Intersection between {} and {} is {:?}",
            left_compartment, right_compartment, intersect
        );
        panic!("Intersection is not 1");
    }

    *intersect.iter().next().unwrap()
}

fn compute_priority(c: char) -> usize {
    if c.is_uppercase() {
        c as usize - 'A' as usize + 27
    } else {
        c as usize - 'a' as usize + 1
    }
}

fn part_one(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| compute_priority(find_common(line)))
        .sum()
}

fn part_two(lines: &[&str]) -> usize {
    lines
        .chunks(3)
        .map(|group| {
            let common_item = group
                .iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(b))
                .expect("Should be one item");
            if common_item.len() != 1 {
                println!("Not exactly 1 common item: {:?}", common_item);
                panic!();
            }
            compute_priority(*common_item.iter().next().unwrap())
        })
        .sum::<usize>()
}
