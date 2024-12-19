use std::collections::{HashMap, HashSet, VecDeque};

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let towels = input.lines().next().unwrap().split(", ").collect();

    let patterns = input.lines().skip(2).collect();

    (towels, patterns)
}

fn is_possible(pattern: &str, towels: &Vec<&str>) -> bool {
    let mut possible_substrings = vec![false; pattern.len() + 1];
    possible_substrings[0] = true;

    for i in 0..pattern.len() {
        for &t in towels {
            if t.len() > pattern.len() - i {
                continue;
            }
            if possible_substrings[i] && pattern[i..i + t.len()] == *t {
                possible_substrings[i + t.len()] = true;
            }
        }
    }

    possible_substrings[pattern.len()]
}

fn possible_ways(pattern: &str, towels: &Vec<&str>) -> u64 {
    let mut possible_solutions: Vec<u64> = vec![0; pattern.len() + 1];
    possible_solutions[0] = 1;

    for i in 0..pattern.len() {
        for &t in towels {
            if t.len() > pattern.len() - i {
                continue;
            }

            if possible_solutions[i] > 0 && pattern[i..i + t.len()] == *t {
                possible_solutions[i + t.len()] += possible_solutions[i];
            }
        }
    }

    possible_solutions[pattern.len()]
}

fn possible_patterns(towels: &Vec<&str>, patterns: &Vec<&str>) -> u64 {
    patterns.iter().filter( |pattern| is_possible(pattern, towels)).count() as u64
}

fn sum_of_possible_ways(towels: &Vec<&str>, patterns: &Vec<&str>) -> u64 {
    patterns.iter().map(|pattern| possible_ways(pattern, towels)).sum()
}

pub fn part1(input: &str) -> u64 {
    let (towels, patterns) = parse_input(input);
    possible_patterns(&towels, &patterns)
}

pub fn part2(input: &str) -> u64 {
    let (towels, patterns) = parse_input(input);
    sum_of_possible_ways(&towels, &patterns)
}
