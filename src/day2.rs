use aoc_runner_derive::aoc;
use anyhow::{Context, Result};
use itertools::Itertools;

fn is_safe<'a, I>(report: I) -> bool
where
    I: Iterator<Item = u64>
{
    let mut iter = report.tuple_windows();

    let mut direction: Option<i64> = None;
    iter.all(|(i, j)| {
        if direction.is_none() {
            direction = if (j as i64 - i as i64) < 0 { Some(-1) } else { Some(1) }
        }

        let diff = (j - i) as i64 * direction.unwrap();
        diff <= 3 && diff >= 1
    })
}

fn is_safe_with_damper(report: &[u64]) -> bool {
    (0..report.len()).any(|skip| {
        let iter = report.into_iter()
            .enumerate()
            .filter(|&(i, _)| i != skip)
            .map(|(_, v)| *v);

        is_safe(iter)
    })
}

fn input_iter<'a>(input: &'a str) -> impl Iterator<Item = impl Iterator<Item = u64> + 'a> + 'a {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|i| i.parse().unwrap())
    })
}

fn parse_input(input: &str) -> Result<Vec<Vec<u64>>> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|i| i.parse().context("parse error"))
            .collect()
    }).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    let input = input_iter(input);
    input.filter_map(|report| if is_safe(report) { Some(()) } else { None }).count().try_into().unwrap()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    let input = parse_input(input).unwrap();
    input.iter().filter(|report| is_safe_with_damper(report)).count().try_into().unwrap()
}

