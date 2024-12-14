use anyhow::{Context, Result};
use aoc_runner_derive::aoc;


#[derive(Debug)]
struct Equation {
    test_value: u64,
    values: Vec<u64>,
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

impl TryFrom<&str> for Equation {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut split = value.split(": ");
        let (test_value, values) = (
            split.next().context("invalid equation")?.parse()?,
            split.next().context("invalid equation")?.split(' ').map(|v| v.parse().context("parse error")).collect::<Result<Vec<_>>>()?,
        );
        Ok(Self { test_value, values })
    }
}

fn valid_equation(equation: &Equation, allowed_operations: &[Operation]) -> bool {
    let calculated_value = equation.values[0];

    allowed_operations.iter().any(|operation| {
        valid_inner(calculated_value, &equation.values[1..], operation, allowed_operations, equation.test_value)
    })
}

fn valid_inner(acc: u64, remaining_values: &[u64], next_operation: &Operation, allowed_operations: &[Operation], expected_value: u64) -> bool {
    if remaining_values.len() == 0 {
        return acc == expected_value;
    }

    let acc = match next_operation {
        Operation::Add => acc + remaining_values[0],
        Operation::Multiply => acc * remaining_values[0],
        Operation::Concatenate => {
            let mut concatenated = acc.to_string();
            concatenated.push_str(&remaining_values[0].to_string());
            concatenated.parse().unwrap()
        }
    };

    allowed_operations.iter().any(|operation| {
        valid_inner(acc, &remaining_values[1..], operation, allowed_operations, expected_value)
    })
}

fn sum_of_valid_test_values(equations: &[Equation], allowed_operations: &[Operation]) -> u64 {
    equations.iter().fold(0, |acc, equation| {
        if valid_equation(equation, allowed_operations) {
            acc + equation.test_value
        } else {
            acc
        }
    })
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let equations = input.lines().map( |line| line.try_into()).collect::<Result<Vec<_>>>().unwrap();
    sum_of_valid_test_values(&equations, &vec![Operation::Add, Operation::Multiply])
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let equations = input.lines().map( |line| line.try_into()).collect::<Result<Vec<_>>>().unwrap();
    sum_of_valid_test_values(&equations, &vec![Operation::Add, Operation::Multiply, Operation::Concatenate])
}
