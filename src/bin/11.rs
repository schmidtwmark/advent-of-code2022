#![feature(anonymous_lifetime_in_impl_trait)]
use aoc::Solver;
use itertools::Itertools;
use log::debug;
use sscanf::scanf;

type Item = u128;

enum Operation {
    Add(u128),
    Multiply(u128),
    Square,
}

impl Operation {
    fn new(line: &str) -> Operation {
        let (operator, operand) =
            scanf!(line.trim(), "Operation: new = old {} {}", String, String).unwrap();
        match (&*operator, &*operand) {
            ("+", _) => Operation::Add(operand.parse().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", _) => Operation::Multiply(operand.parse().unwrap()),
            (_, _) => panic!("Unknown operation: {}", line),
        }
    }
}

struct Test {
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize,
}
impl Test {
    fn new(lines: &[&str]) -> Test {
        let divisor_line = lines[0];
        let true_monkey_line = lines[1];
        let false_monkey_line = lines[2];
        Test {
            divisor: scanf!(divisor_line.trim(), "Test: divisible by {}", u128).unwrap(),
            true_monkey: scanf!(
                true_monkey_line.trim(),
                "If true: throw to monkey {}",
                usize
            )
            .unwrap(),
            false_monkey: scanf!(
                false_monkey_line.trim(),
                "If false: throw to monkey {}",
                usize
            )
            .unwrap(),
        }
    }
}
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test: Test,
    inspection_count: usize,
}

impl Monkey {
    fn new(lines: &[&str]) -> Monkey {
        debug!("Processing group: {:?}", lines.iter().collect_vec());
        let items_line = lines[1];
        let operation_line = lines[2];
        let test = Test::new(&lines[3..6]);
        let operation = Operation::new(operation_line);

        let items = items_line
            .trim()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect_vec();

        Monkey {
            items,
            operation,
            test,
            inspection_count: 0,
        }
    }

    fn exec(&mut self, divide: bool, superdivisor: u128) -> Vec<(Item, usize)> {
        self.inspection_count += self.items.len();

        let passing_items = self
            .items
            .iter()
            .map(|item| {
                let mut new_item = match self.operation {
                    Operation::Add(operand) => item + operand,
                    Operation::Multiply(operand) => item * operand,
                    Operation::Square => item * item,
                };
                if divide {
                    new_item /= 3;
                }
                new_item %= superdivisor;
                let target = if new_item % self.test.divisor == 0 {
                    self.test.true_monkey
                } else {
                    self.test.false_monkey
                };
                (new_item, target)
            })
            .collect_vec();

        self.items.clear();

        passing_items
    }
}

fn simulate(monkeys: &mut [Monkey], round: usize, divide: bool, superdivisor: u128) {
    for monkey_idx in 0..monkeys.len() {
        let monkey = &mut monkeys[monkey_idx];
        let passing_items = monkey.exec(divide, superdivisor);
        for (item, target) in passing_items {
            monkeys[target].items.push(item);
        }
    }

    debug!("After round {}, monkeys are holding:", round);
    for (monkey_idx, monkey) in monkeys.iter().enumerate() {
        debug!("Monkey {}: {:?}", monkey_idx, monkey.items);
    }
}

struct Solution {}
impl Solver<'_, usize> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> usize {
        let mut monkeys = lines.split(|s| s.is_empty()).map(Monkey::new).collect_vec();

        let superdivisor: u128 = monkeys.iter().map(|m| m.test.divisor).product();
        debug!("Superdivisor: {}", superdivisor);
        for round in 1..=20 {
            simulate(&mut monkeys, round, true, superdivisor);
        }

        for (monkey_idx, monkey) in monkeys.iter().enumerate() {
            debug!(
                "Monkey {} inspected {} items",
                monkey_idx, monkey.inspection_count
            );
        }

        monkeys
            .iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .product()
    }

    fn solve_part_two(&self, lines: &[&str]) -> usize {
        let mut monkeys = lines.split(|s| s.is_empty()).map(Monkey::new).collect_vec();

        let superdivisor: u128 = monkeys.iter().map(|m| m.test.divisor).product();
        debug!("Superdivisor: {}", superdivisor);

        for round in 1..=10000 {
            simulate(&mut monkeys, round, false, superdivisor);
        }

        for (monkey_idx, monkey) in monkeys.iter().enumerate() {
            debug!(
                "Monkey {} inspected {} items",
                monkey_idx, monkey.inspection_count
            );
        }

        monkeys
            .iter()
            .map(|m| m.inspection_count)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

fn main() {
    let sample = include_str!("../../samples/11.txt");
    let input = include_str!("../../inputs/11.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 10605),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, 2713310158), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
