use std::fmt::Display;

use aoc::Solver;
use itertools::Itertools;
use log::{debug, error, info};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
    Subtract,
    Divide,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Subtract => write!(f, "-"),
            Operator::Divide => write!(f, "/"),
        }
    }
}

type MonkeyId = String;

type MonkeyMap = hashbrown::HashMap<MonkeyId, Monkey>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Monkey {
    Constant(i128),
    Operation(Operator, MonkeyId, MonkeyId),
    Human,
}

fn parse_lines(lines: &[&str]) -> MonkeyMap {
    let mut map = MonkeyMap::new();
    for line in lines {
        let (lhs, rhs) = line.split(": ").collect_tuple().unwrap();
        let monkey_id = lhs.to_string();

        let monkey = if let Ok(constant) = rhs.parse::<i128>() {
            Monkey::Constant(constant)
        } else {
            let (lhs, op, rhs) = rhs.split(' ').collect_tuple().unwrap();
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

fn process(monkey: &Monkey, map: &MonkeyMap, human_value: i128) -> i128 {
    match monkey {
        Monkey::Constant(c) => *c,
        Monkey::Operation(op, lhs, rhs) => {
            let lhs = process(map.get(lhs).unwrap(), map, human_value);
            let rhs = process(map.get(rhs).unwrap(), map, human_value);
            match op {
                Operator::Add => lhs + rhs,
                Operator::Multiply => lhs * rhs,
                Operator::Subtract => lhs - rhs,
                Operator::Divide => lhs / rhs,
            }
        }
        Monkey::Human => human_value,
    }
}

enum Expression {
    Constant(i128),
    Variable(String, i128),
    Operation(Operator, Box<Expression>, Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Constant(c) => write!(f, "{}", c),
            Expression::Variable(v, factor) => write!(f, "{}{}", factor, v),
            Expression::Operation(op, lhs, rhs) => {
                write!(f, "({} {} {})", lhs, op, rhs)
            }
        }
    }
}

fn to_expression(monkey: &Monkey, map: &MonkeyMap) -> Expression {
    match monkey {
        Monkey::Constant(c) => Expression::Constant(*c),
        Monkey::Operation(op, lhs, rhs) => {
            let lhs = to_expression(map.get(lhs).unwrap(), map);
            let rhs = to_expression(map.get(rhs).unwrap(), map);
            Expression::Operation(*op, Box::new(lhs), Box::new(rhs))
        }
        Monkey::Human => Expression::Variable("x".to_string(), 1),
    }
}

impl Expression {
    fn evaluate(&self, x: i128) -> i128 {
        match self {
            Expression::Constant(c) => *c,
            Expression::Variable(_, factor) => x * factor,
            Expression::Operation(op, lhs, rhs) => {
                let lhs = lhs.evaluate(x);
                let rhs = rhs.evaluate(x);
                match op {
                    Operator::Add => lhs + rhs,
                    Operator::Multiply => lhs * rhs,
                    Operator::Subtract => lhs - rhs,
                    Operator::Divide => lhs / rhs,
                }
            }
        }
    }

    fn simplify(&self) -> Expression {
        match self {
            Expression::Constant(c) => Expression::Constant(*c),
            Expression::Variable(v, factor) => Expression::Variable(v.clone(), *factor),
            Expression::Operation(op, lhs, rhs) => {
                let lhs = lhs.simplify();
                let rhs = rhs.simplify();

                match (op, &lhs, &rhs) {
                    (Operator::Add, Expression::Constant(0), _) => rhs,
                    (Operator::Add, _, Expression::Constant(0)) => lhs,
                    (Operator::Multiply, Expression::Constant(0), _) => Expression::Constant(0),
                    (Operator::Multiply, _, Expression::Constant(0)) => Expression::Constant(0),
                    (Operator::Multiply, Expression::Constant(1), _) => rhs,
                    (Operator::Multiply, _, Expression::Constant(1)) => lhs,
                    (Operator::Subtract, _, Expression::Constant(0)) => lhs,
                    (Operator::Divide, _, Expression::Constant(1)) => lhs,

                    (_, Expression::Constant(_), Expression::Constant(_)) => {
                        Expression::Constant(self.evaluate(0))
                    }
                    _ => Expression::Operation(*op, Box::new(lhs), Box::new(rhs)),
                }
            }
        }
    }
}

fn solve(left_expr: &Expression, right_expr: &Expression) -> Option<i128> {
    let (mut left_bound, mut right_bound) = (-10000000000000, 10000000000000);
    while left_bound + 1 < right_bound {
        let i = (left_bound + right_bound) / 2;
        let left = left_expr.evaluate(i);
        let right = right_expr.evaluate(i);
        let ordering = left.cmp(&right);
        debug!("x = {} => ({} {:?} {})", i, left, ordering, right);
        match ordering {
            Ordering::Less => {
                right_bound = i;
            }
            Ordering::Equal => {
                return Some(i);
            }
            Ordering::Greater => {
                left_bound = i;
            }
        }
    }
    None
}

struct Solution {}
impl Solver<'_, i128> for Solution {
    fn solve_part_one(&self, lines: &[&str]) -> i128 {
        let monkeys = parse_lines(lines);

        process(&monkeys["root"], &monkeys, 0)
    }

    fn solve_part_two(&self, lines: &[&str]) -> i128 {
        let monkeys = {
            let mut t = parse_lines(lines);
            *t.get_mut("humn").unwrap() = Monkey::Human;
            t
        };

        let root = &monkeys["root"];

        let (left, right) = if let Monkey::Operation(_, lhs, rhs) = root {
            (lhs, rhs)
        } else {
            panic!("Root is not an operation");
        };

        let left = &monkeys[left];
        let right = &monkeys[right];

        let left_expression = to_expression(left, &monkeys);
        let left_simplified = left_expression.simplify();
        let right_expression = to_expression(right, &monkeys);
        let right_simplified = right_expression.simplify();

        assert_eq!(right_simplified.evaluate(0), right_expression.evaluate(0));

        assert_eq!(left_simplified.evaluate(0), left_expression.evaluate(0));
        assert_eq!(left_simplified.evaluate(100), left_expression.evaluate(100));
        assert_eq!(
            left_simplified.evaluate(-100),
            left_expression.evaluate(-100)
        );

        info!("Left: {}", left_expression);
        info!("Left: {}", left_simplified);
        info!("Right: {}", right_expression);
        info!("Right: {}", right_simplified);

        if let Some(solution) = solve(&left_simplified, &right_simplified) {
            return solution;
        }
        if let Some(solution) = solve(&right_simplified, &left_simplified) {
            return solution;
        }

        error!("Failed to find a solution");
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
        aoc::Input::new_sample(sample, 301),
        aoc::Input::new_final(input),
    ];

    Solution {}.run(&part_one_problems, &part_two_problems);
}
