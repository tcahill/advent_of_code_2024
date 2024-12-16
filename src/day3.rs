use anyhow::Result;
use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    re.captures_iter(input).map(|capture| {
        capture[1].parse::<u64>().unwrap_or(0) * capture[2].parse::<u64>().unwrap_or(0)
    }).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    for capture in re.captures_iter(input) {
        match &capture[0][..3] {
            "do(" => { enabled = true; },
            "don" => { enabled = false; },
            "mul" => {
                if enabled {
                    sum += capture[1].parse::<u64>().unwrap_or(0) * capture[2].parse::<u64>().unwrap_or(0);
                }
            },
            _ => {},
        }
    }

    sum
}
