use itertools::Itertools;
use std::fs;
use std::thread;
use std::time::Instant;

fn get_filename(sample: bool) -> String {
    let current_exe = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    if sample {
        format!("samples/{}.txt", current_exe)
    } else {
        format!("inputs/{}.txt", current_exe)
    }
}

fn get_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    contents.lines().map(|s| s.to_owned()).collect_vec()
}

pub fn run_all<F1, F2>(part1: F1, part2: F2, sample_solution_one: usize, sample_solution_two: usize)
where
    F1: Fn(&[String]) -> usize + Sync + 'static,
    F2: Fn(&[String]) -> usize + Sync + 'static,
{
    let sample = get_lines(&get_filename(true));
    let real = get_lines(&get_filename(false));

    thread::scope(|s| {
        s.spawn(|| {
            let result = part1(&sample);
            assert_eq!(result, sample_solution_one);
        });
        s.spawn(|| {
            let start = Instant::now();
            let result = part1(&real);
            println!("Part one: {:?}, took {:?}", result, start.elapsed());
        });
        s.spawn(|| {
            let result = part2(&sample);
            assert_eq!(result, sample_solution_two);
        });
        s.spawn(|| {
            let start = Instant::now();
            let result = part2(&real);
            println!("Part two: {:?}, took {:?}", result, start.elapsed());
        });
    });
}
