use aoc::Solver;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Subtract,
    Divide,
}

type MonkeyId = String;

type MonkeyMap = hashbrown::HashMap<MonkeyId, Monkey>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Monkey {
    Constant(i64),
    Operation(Operator, MonkeyId, MonkeyId),
}

fn parse_lines(lines: &[&str]) -> MonkeyMap {
    let mut map = MonkeyMap::new();
    for line in lines {
        let (lhs, rhs) = line.split(": ").collect_tuple().unwrap();
        let monkey_id = lhs.to_string();

        let monkey = if let Ok(constant) = rhs.parse::<i64>() {
            Monkey::Constant(constant)
        } else {
            let (lhs, op, rhs) = rhs.split(" ").collect_tuple().unwrap();
            let op = match op {
                "+" => Operator::Add,
                "*" => Operator::Multiply,
                "-" => Operator::Subtract,
                "/" => Operator::Divide,
                _ => panic!("Unknown operator: {}", op),
            };
            Monkey::Operation(op, lhs.to_string(), rhs.to_string())
        };

        map.insert(monkey_id, monkey);
    }
    map
}

fn process(monkey: &Monkey, map: &MonkeyMap) -> i64 {
    match monkey {
        Monkey::Constant(c) => *c,
        Monkey::Operation(op, lhs, rhs) => {
            let lhs = process(map.get(lhs).unwrap(), map);
            let rhs = process(map.get(rhs).unwrap(), map);
            match op {
                Operator::Add => lhs + rhs,
                Operator::Multiply => lhs * rhs,
                Operator::Subtract => lhs - rhs,
                Operator::Divide => lhs / rhs,
            }
        }
    }
}

struct Solution {}
impl Solver<'_, i64> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> i64 {
        let monkeys = parse_lines(lines);

        process(&monkeys["root"], &monkeys)
    }

    fn solve_part_two(&self, lines: &[&str]) -> i64 {
        Default::default()
    }
}

fn main() {
    let sample = include_str!("../../samples/21.txt");
    let input = include_str!("../../inputs/21.txt");
    let part_one_problems = [
        aoc::Input::new_sample(sample, 152),
        aoc::Input::new_final(input),
    ];

    let part_two_problems = [
        aoc::Input::new_sample(sample, Default::default()), // TODO: Fill in expected sample result
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
