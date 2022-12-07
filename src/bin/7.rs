use std::{collections::VecDeque, convert::TryFrom};

use aoc::Solver;
use itertools::Itertools;
use log::debug;
use std::collections::HashMap;

enum Command {
    ChangeDirectory(Option<String>),
    ListDirectory,
}

enum Output {
    Command(Command),
    File { name: String, size: usize },
    Directory { name: String },
}

#[derive(Debug)]
enum AOCError {
    InvalidCommand(String),
    InvalidOutput(String),
}

impl TryFrom<&str> for Command {
    type Error = AOCError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();

        let leader = parts.next().unwrap(); // Skip the $
        if leader != "$" {
            return Err(AOCError::InvalidCommand(format!(
                "Line {leader} must start with $"
            )));
        }

        let command = parts.next().unwrap();
        let args = parts.next();
        match command {
            "cd" => Ok(Self::ChangeDirectory(
                args.filter(|s| *s != "..").map(|s| s.to_string()),
            )),
            "ls" => Ok(Self::ListDirectory),
            _ => Err(AOCError::InvalidCommand(format!(
                "Unknown command {command}"
            ))),
        }
    }
}

impl TryFrom<&str> for Output {
    type Error = AOCError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(command) = Command::try_from(value) {
            return Ok(Self::Command(command));
        }

        let (a, b) = value.split_whitespace().collect_tuple().unwrap();
        match a {
            "dir" => Ok(Self::Directory {
                name: b.to_string(),
            }),
            _ => Ok(Self::File {
                name: b.to_string(),
                size: a
                    .parse::<usize>()
                    .map_err(|e| AOCError::InvalidOutput(e.to_string()))?,
            }),
        }
    }
}

fn process(lines: &[&str]) -> Vec<Output> {
    lines
        .iter()
        .map(|line| Output::try_from(*line).unwrap())
        .collect()
}

#[derive(Debug)]
enum File {
    Directory(Box<Directory>),
    File { size: usize },
}

#[derive(Debug)]
struct Directory {
    files: HashMap<String, File>,
    total_size: usize,
}

impl Directory {
    fn fill_in_total_size(&mut self) -> usize {
        let mut total_size = 0;
        for file in self.files.values_mut() {
            total_size += match file {
                File::Directory(dir) => dir.fill_in_total_size(),
                File::File { size, .. } => *size,
            };
        }
        self.total_size = total_size;
        total_size
    }

    fn get_directories(&self) -> Vec<&Box<Directory>> {
        let mut directories = Vec::new();
        for file in self.files.values() {
            if let File::Directory(dir) = file {
                directories.push(dir);
                directories.extend(dir.get_directories());
            }
        }
        directories
    }
}

fn build_filesystem(output: Vec<Output>) -> Box<Directory> {
    let mut current_dir = VecDeque::new();
    let mut root = Box::new(Directory {
        files: HashMap::new(),
        total_size: 0,
    });
    // skip the first line, we don't need to go to root

    for line in output.into_iter().skip(1) {
        match line {
            Output::Command(command) => match command {
                Command::ChangeDirectory(dir) => {
                    if let Some(target) = dir {
                        current_dir.push_back(target);
                    } else {
                        current_dir.pop_back();
                    }
                    debug!("At directory {:?}", current_dir);
                }
                Command::ListDirectory => {
                    debug!("Listing directory {:?}", current_dir);
                }
            },
            Output::File { name, size } => {
                debug!("Adding file {name} to {:?}", current_dir);
                let mut local_directory = &mut root;
                for path in &current_dir {
                    local_directory = match local_directory.files.get_mut(path).unwrap() {
                        File::Directory(dir) => dir,
                        _ => panic!("Expected directory"),
                    };
                }

                local_directory.files.insert(name, File::File { size });
            }
            Output::Directory { name } => {
                debug!("Adding directory {name} to {:?}", current_dir);
                let mut local_directory = &mut root;
                for path in &current_dir {
                    local_directory = match local_directory.files.get_mut(path).unwrap() {
                        File::Directory(dir) => dir,
                        _ => panic!("Expected directory"),
                    };
                }
                local_directory.files.insert(
                    name,
                    File::Directory(Box::new(Directory {
                        files: HashMap::new(),
                        total_size: 0,
                    })),
                );
            }
        }
    }

    root
}

struct Solution {}
impl Solver<usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let output = process(lines);
        let mut root = build_filesystem(output);
        root.fill_in_total_size();
        let directories = root.get_directories();
        let filtered_directories = directories
            .into_iter()
            .filter(|dir| dir.total_size <= 100000)
            .collect_vec();

        filtered_directories
            .into_iter()
            .map(|dir| dir.total_size)
            .sum()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let output = process(lines);
        let mut root = build_filesystem(output);
        root.fill_in_total_size();

        let unused_space = 70000000 - root.total_size;
        let target = 30000000 - unused_space;

        debug!("Seeking directory with at least {} bytes", target);
        debug!("Root: {:#?}", root);

        let directories = root.get_directories();
        let filtered_directories = directories
            .into_iter()
            .filter(|dir| dir.total_size >= target)
            .collect_vec();

        debug!("Filtered directories: {:?}", filtered_directories);

        filtered_directories
            .into_iter()
            .map(|dir| dir.total_size)
            .min()
            .unwrap()
    }
}

fn main() {
    let sample = include_str!("../../samples/7.txt");
    let input = include_str!("../../inputs/7.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 95437),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 24933642),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
