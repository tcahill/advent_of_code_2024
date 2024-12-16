use aoc_runner_derive::aoc;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::iter::zip;

fn distance(list1: &mut [u64], list2: &mut [u64]) -> u64 {
    list1.sort_unstable();
    list2.sort_unstable();

    zip(list1, list2).map(|elem| elem.1.abs_diff(*elem.0)).sum()
}

fn similarity(list1: &[u64], list2: &[u64]) -> u64 {
    let counts = list2.iter().counts();

    list1.iter().map(|elem| {
        let count = counts.get(elem).unwrap_or(&0);
        elem * *count as u64
    }).sum()
}

fn parse_input(input: &str) -> Result<(Vec<u64>, Vec<u64>)> {
    input.split_ascii_whitespace()
        .map(|elem| elem.parse::<u64>().context("parse error"))
        .tuples::<(_, _)>()
        .map(|(a,b)| Ok((a?, b?)))
        .process_results(|tuple| tuple.unzip())
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u64 {
    let (mut list1, mut list2) = parse_input(input).unwrap();
    distance(&mut list1, &mut list2)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let (list1, list2) = parse_input(input).unwrap();
    similarity(&list1, &list2)
}
